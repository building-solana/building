use crate::block::SolanaBlock;
use anyhow::Result;
use log::info;
use sha2::{Digest, Sha256};

pub fn build_block(slot: u64, previous_hash: Option<&str>) -> Result<SolanaBlock> {
    info!("Building block for slot: {}", slot);
    let mut hasher = Sha256::new();

    hasher.update(slot.to_le_bytes());

    if let Some(prev) = previous_hash {
        hasher.update(prev.as_bytes());
    } else {
        hasher.update(b"default");
    }

    let hash_result = hasher.finalize();
    let blockhash = format!("{:x}", hash_result);

    info!("Built block with hash: {}", blockhash);
    Ok(SolanaBlock { slot, blockhash })
}
