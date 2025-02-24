use std::{
    collections::HashSet,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{Builder, JoinHandle},
    time::{Duration, Instant, SystemTime},
};

use cached::{Cached, TimedCache};
use dashmap::DashMap;
use log::{error, info, warn};
use prost_types::Timestamp;
use tokio::{
    runtime::Runtime,
    select,
    sync::mpsc::{channel, Receiver, Sender},
    time::{interval, sleep},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    transport::{Channel, Endpoint},
    Response, Status, Streaming,
};

pub struct EngineConfig {
    pub engine_url: String,
    pub auth_service_url: String,
}

pub struct EngineRelayerHandler {
    engine_forwarder: Option<JoinHandle<()>>,
}

impl EngineRelayerHandler {
    pub fn new(
        engine_config: Option<EngineConfig>,
        mut engine_receiver: Receiver<Vec<u8>>,
        exit: Arc<AtomicBool>,
    ) -> EngineRelayerHandler {
        let engine_forwarder = engine_config.map(|config| {
            Builder::new()
                .name("engine_relayer_handler_thread".into())
                .spawn(move || {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async move {
                        while !exit.load(Ordering::Relaxed) {
                            let result = Self::auth_and_connect(
                                &config.engine_url,
                                &config.auth_service_url,
                                &mut engine_receiver,
                                &exit,
                            )
                            .await;

                            if let Err(e) = result {
                                error!("Error connecting: {:?}", e);
                                sleep(Duration::from_secs(2)).await;
                            }
                        }
                    });
                })
                .unwrap()
        });

        EngineRelayerHandler { engine_forwarder }
    }

    pub fn join(self) {
        if let Some(forwarder) = self.engine_forwarder {
            forwarder.join().unwrap()
        }
    }

    async fn auth_and_connect(
        engine_url: &str,
        auth_service_url: &str,
        engine_receiver: &mut Receiver<Vec<u8>>,
        exit: &Arc<AtomicBool>,
    ) -> Result<(), String> {
        let endpoint = Endpoint::from_str(engine_url).map_err(|e| e.to_string())?;
        let channel = endpoint.connect().await.map_err(|e| e.to_string())?;

        let mut heartbeat_interval = interval(Duration::from_millis(500));
        while !exit.load(Ordering::Relaxed) {
            select! {
                _ = heartbeat_interval.tick() => {
                    info!("Sending heartbeat");
                }
                maybe_packet = engine_receiver.recv() => {
                    if let Some(packet) = maybe_packet {
                        info!("Received packet, forwarding...");
                    }
                }
            }
        }
        Ok(())
    }
}
