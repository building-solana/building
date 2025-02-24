use anyhow::{anyhow, Result};
use log::info;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::RpcBlock;
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::CommitmentConfig;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SolanaBlock {
    pub slot: Slot,
    pub blockhash: String,
}
pub fn fetch_block(rpc_url: &str, slot: Slot) -> Result<SolanaBlock> {
    info!("Connecting to Solana RPC at {}", rpc_url);
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let rpc_block: RpcBlock = client
        .get_block(slot)
        .map_err(|e| anyhow!("Failed to fetch block at slot {}: {:?}", slot, e))?;

    let blockhash = rpc_block.blockhash;
    info!("Fetched block at slot: {} with hash: {}", slot, blockhash);

    Ok(SolanaBlock { slot, blockhash })
}
