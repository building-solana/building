use crate::block::SolanaBlock;
use anyhow::{anyhow, Result};
use log::info;
use serde_json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use solana_sdk::transaction::Transaction;

const MEMO_PROGRAM_ID: &str = "Memo111111111111111111111111111111111111111";

pub fn send_block(rpc_url: &str, block: &SolanaBlock, keypair_path: &str) -> Result<()> {
    info!("Connecting to Solana RPC at {}", rpc_url);
    let client = RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let keypair = read_keypair_file(keypair_path)
        .map_err(|e| anyhow!("Failed to read keypair file {}: {:?}", keypair_path, e))?;
    let payer = keypair.pubkey();

    let block_data = serde_json::to_string(block)
        .map_err(|e| anyhow!("Failed to serialize block data: {:?}", e))?;
    info!("Serialized block data: {}", block_data);

    let memo_program_id = MEMO_PROGRAM_ID
        .parse::<Pubkey>()
        .map_err(|e| anyhow!("Invalid memo program ID: {:?}", e))?;
    let memo_instruction = Instruction {
        program_id: memo_program_id,
        accounts: vec![],
        data: block_data.into_bytes(),
    };

    let message = Message::new(&[memo_instruction], Some(&payer));

    let recent_blockhash = client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Failed to get recent blockhash: {:?}", e))?;
    let transaction = Transaction::new(&[&keypair], message, recent_blockhash);

    let signature = client
        .send_and_confirm_transaction(&transaction)
        .map_err(|e| anyhow!("Failed to send transaction: {:?}", e))?;
    info!(
        "Block sent successfully with transaction signature: {}",
        signature
    );

    Ok(())
}
