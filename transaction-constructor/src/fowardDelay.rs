use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{Builder, JoinHandle},
    time::{Duration, Instant, SystemTime},
};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use log::{error, info, warn};
tokio::sync::mpsc::error::TrySendError;

pub const FORWARDER_QUEUE_CAPACITY: usize = 5_000;

/// Handles forwarding and delaying packets before they reach the validator.
pub fn start_forward_and_delay_thread(
    verified_receiver: Receiver<Vec<u8>>, 
    delay_packet_sender: Sender<Vec<u8>>, 
    packet_delay_ms: u32,
    block_engine_sender: tokio::sync::mpsc::Sender<Vec<u8>>, 
    num_threads: u64,
    disable_mempool: bool,
    exit: &Arc<AtomicBool>,
) -> Vec<JoinHandle<()>> {
    const SLEEP_DURATION: Duration = Duration::from_millis(5);
    let packet_delay = Duration::from_millis(packet_delay_ms as u64);

    (0..num_threads)
        .map(|thread_id| {
            let verified_receiver = verified_receiver.clone();
            let delay_packet_sender = delay_packet_sender.clone();
            let block_engine_sender = block_engine_sender.clone();
            let exit = exit.clone();
            
            Builder::new()
                .name(format!("forwarder_thread_{thread_id}"))
                .spawn(move || {
                    let mut buffered_packets: VecDeque<(Instant, Vec<u8>)> = VecDeque::new();
                    let metrics_interval = Duration::from_secs(1);
                    let mut last_metrics_upload = Instant::now();

                    while !exit.load(Ordering::Relaxed) {
                        if last_metrics_upload.elapsed() >= metrics_interval {
                            last_metrics_upload = Instant::now();
                        }

                        match verified_receiver.recv_timeout(SLEEP_DURATION) {
                            Ok(packet) => {
                                let instant = Instant::now();
                                let system_time = SystemTime::now();
                                
                                if !disable_mempool {
                                    match block_engine_sender.try_send(packet.clone()) {
                                        Ok(_) => {},
                                        Err(TrySendError::Closed(_)) => panic!("Error sending to block engine"),
                                        Err(TrySendError::Full(_)) => warn!("Block engine queue full"),
                                    }
                                }
                                buffered_packets.push_back((instant, packet));
                            }
                            Err(RecvTimeoutError::Timeout) => {}
                            Err(RecvTimeoutError::Disconnected) => panic!("Receiver disconnected"),
                        }

                        while let Some((timestamp, packet)) = buffered_packets.front() {
                            if timestamp.elapsed() < packet_delay {
                                break;
                            }
                            let _ = buffered_packets.pop_front();
                            delay_packet_sender.send(packet.clone()).expect("Failed to send delayed packet");
                        }
                    }
                })
                .unwrap()
        })
        .collect()
}
