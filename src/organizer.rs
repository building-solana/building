use crate::block::SolanaBlock;
use anyhow::Result;
use log::info;

pub fn organize_blocks(mut blocks: Vec<SolanaBlock>) -> Result<Vec<SolanaBlock>> {
    info!("Organizing {} blocks", blocks.len());
    blocks.sort_by_key(|b| b.slot);
    Ok(blocks)
}

pub fn print_blocks(blocks: &[SolanaBlock]) {
    info!("Printing organized blocks:");
    for block in blocks {
        println!("Slot: {}, Blockhash: {}", block.slot, block.blockhash);
    }
}
