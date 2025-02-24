use builder_block::{
    auth::{AuthServiceImpl, ValidatorAuther},
    health::HealthManager,
    network::{get_public_ip_addr, multi_bind_in_range},
    relayer::{LeaderScheduleCacheUpdater, RelayerImpl},
    rpc::LoadBalancer,
    tpu::{Tpu, TpuSockets},
};
use clap::Parser;
use crossbeam_channel::tick;
use dashmap::DashMap;
use env_logger::Env;
use log::{debug, error, info, warn};
use openssl::{hash::MessageDigest, pkey::PKey};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
};
use std::{
    collections::HashSet,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    ops::Range,
    path::PathBuf,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use tokio::{runtime::Builder, signal};
use tonic::transport::Server;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, env, default_value_t = 11_228)]
    tpu_quic_port: u16,

    #[arg(long, env, default_value_t = 1)]
    num_tpu_quic_servers: u16,

    #[arg(long, env)]
    keypair_path: PathBuf,

    #[arg(long, env, value_delimiter = ' ')]
    rpc_servers: Vec<String>,

    #[arg(long, env, value_delimiter = ' ')]
    websocket_servers: Vec<String>,
}

fn main() {
    env_logger::Builder::from_env(Env::new().default_filter_or("info")).init();
    let args: Args = Args::parse();
    info!("args: {:?}", args);

    let public_ip = get_public_ip_addr().expect("Failed to determine public IP");
    info!("public ip: {:?}", public_ip);

    let keypair = Arc::new(read_keypair_file(&args.keypair_path).expect("Keypair file not found"));
    info!("Relayer started with pubkey: {}", keypair.pubkey());

    let exit = Arc::new(AtomicBool::new(false));
    let rpc_load_balancer = Arc::new(LoadBalancer::new(&args.rpc_servers, &exit));

    let health_manager = HealthManager::new(exit.clone());
    let leader_cache = LeaderScheduleCacheUpdater::new(&rpc_load_balancer, &exit);
    let relayer_svc = RelayerImpl::new(exit.clone(), leader_cache.handle());

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let auth_svc = AuthServiceImpl::new(ValidatorAuther::default(), exit.clone());
        let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 11226);
        info!("Starting relayer at: {:?}", server_addr);

        Server::builder()
            .add_service(auth_svc.into_service())
            .add_service(relayer_svc.into_service())
            .serve_with_shutdown(server_addr, shutdown_signal(exit.clone()))
            .await
            .expect("Failed to serve relayer");
    });
}

async fn shutdown_signal(exit: Arc<AtomicBool>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };
    tokio::select! {
        _ = ctrl_c => {},
    }
    exit.store(true, Ordering::Relaxed);
    warn!("Signal received, shutting down...");
}
