use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;

/// Sends 1 lamport from your wallet to yourself.
/// Expects a keypair JSON file at ~/.config/solana/id.json (default Solana CLI location).
pub fn send_one_lamport(wallet_pubkey: &str) -> Result<()> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load keypair from default Solana CLI path
    let keypair_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

    let keypair_bytes = std::fs::read_to_string(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair at {:?}: {}", keypair_path, e))?;

    let keypair_vec: Vec<u8> = serde_json::from_str(&keypair_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse keypair JSON: {}", e))?;

    let payer = Keypair::from_bytes(&keypair_vec)
        .map_err(|e| anyhow::anyhow!("Invalid keypair bytes: {}", e))?;

    let to_pubkey = Pubkey::from_str(wallet_pubkey)
        .map_err(|e| anyhow::anyhow!("Invalid wallet address '{}': {}", wallet_pubkey, e))?;

    println!("Sending 1 lamport from {} to {}", payer.pubkey(), to_pubkey);

    // Check balance first
    let balance = rpc_client.get_balance(&payer.pubkey())?;
    println!(
        "Current balance: {} SOL ({} lamports)",
        balance as f64 / LAMPORTS_PER_SOL as f64,
        balance
    );

    if balance < 5000 + 1 {
        anyhow::bail!("Not enough SOL to cover transaction fee + 1 lamport");
    }

    // Build the transfer instruction
    let instruction = system_instruction::transfer(&payer.pubkey(), &to_pubkey, 1);

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash()?;

    // Build and sign the transaction
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Send and confirm
    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction confirmed!");
    println!("Signature: {}", signature);
    println!("Explorer: https://explorer.solana.com/tx/{}", signature);

    Ok(())
}
