use clap::{Parser, Subcommand};
use log::{error, info};

const DEFAULT_RPC_URL: &str = "https://api.devnet.solana.com";

/// CLI tool for interacting with Solana blocks.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Supported subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Fetch blocks from the Solana blockchain.
    Fetch {
        /// Specify a starting slot (optional).
        #[arg(short, long)]
        start: Option<u64>,
    },
    /// Organize fetched blocks.
    Organize {},
    /// Build a new block.
    Build {
        /// The slot number for the new block.
        #[arg(short, long)]
        slot: u64,
        /// Optional previous block hash.
        #[arg(short, long)]
        previous: Option<String>,
    },
    /// Build and send a block to the Solana blockchain.
    Send {
        /// The slot number for the block to build and send.
        #[arg(short, long)]
        slot: u64,
        /// Optional previous block hash.
        #[arg(short, long)]
        previous: Option<String>,
        /// Path to the signer keypair file.
        #[arg(short, long)]
        keypair: String,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    info!("Starting solana-block-builder");

    match &cli.command {
        Some(Commands::Fetch { start }) => {
            info!("Executing fetch command with starting slot: {:?}", start);
            println!("Fetching blocks from the Solana blockchain (placeholder)");
        }
        Some(Commands::Organize {}) => {
            info!("Executing organize command");
            println!("Organizing blocks (placeholder)");
        }
        Some(Commands::Build { slot, previous }) => {
            info!(
                "Executing build command for slot: {} with previous: {:?}",
                slot, previous
            );
            let block = match solana_block_builder::builder::build_block(*slot, previous.as_deref())
            {
                Ok(b) => b,
                Err(e) => {
                    error!("Failed to build block: {:?}", e);
                    return;
                }
            };
            println!(
                "Built block: Slot: {}, Blockhash: {}",
                block.slot, block.blockhash
            );
        }
        Some(Commands::Send {
            slot,
            previous,
            keypair,
        }) => {
            info!(
                "Executing send command for slot: {} with previous: {:?}",
                slot, previous
            );
            let block = match solana_block_builder::builder::build_block(*slot, previous.as_deref())
            {
                Ok(b) => b,
                Err(e) => {
                    error!("Failed to build block: {:?}", e);
                    return;
                }
            };
            match solana_block_builder::sender::send_block(DEFAULT_RPC_URL, &block, keypair) {
                Ok(_) => println!("Block sent successfully."),
                Err(e) => error!("Failed to send block: {:?}", e),
            }
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }
}
