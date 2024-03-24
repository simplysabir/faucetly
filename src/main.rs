use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use tokio::time::{self, Duration};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    wallet_address: String,
    command: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().any(|arg| arg == "save") {
        if let Some(wallet_address) = std::env::args().nth(2) {
            save_wallet_address(&wallet_address).await?;
        } else {
            println!("No wallet address provided.");
        }
    } else {
        start_airdrop_loop().await?;
    }
    Ok(())
}

async fn save_wallet_address(wallet_address: &str) -> Result<()> {
    // Check if the address file already exists to prevent overwriting.
    if std::path::Path::new("wallet_address.txt").exists() {
        return Err(anyhow::anyhow!(
            "Wallet address already saved. To update, delete the existing file."
        ));
    }

    fs::write("wallet_address.txt", wallet_address)
        .with_context(|| "Failed to save wallet address")?;
    println!("Wallet address saved successfully.");
    Ok(())
}

async fn start_airdrop_loop() -> Result<()> {
    let wallet_address = fs::read_to_string("wallet_address.txt")
        .with_context(|| "Failed to read wallet address. Please ensure it is saved.")?;

    println!(
        "Starting airdrop loop for address: {}",
        wallet_address.trim()
    );

    let mut interval = time::interval(Duration::from_secs(86_400));
    loop {
        interval.tick().await;
        std::process::Command::new("solana")
            .args(&["airdrop", "2", wallet_address.trim()])
            .output()
            .expect("Failed to execute command");
        println!("Airdrop executed for address: {}", wallet_address.trim());
    }
}
