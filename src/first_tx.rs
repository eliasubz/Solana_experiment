use crate::utils::wallet;
use anyhow::Result;
use solana_sdk::{signature::Signer, system_instruction, transaction::Transaction};

/// Sends 1 lamport from your wallet to yourself.
pub fn send_one_lamport() -> Result<()> {
    let rpc_client = wallet::rpc_client(true);
    let payer = wallet::load_keypair()?;
    let to_pubkey = payer.pubkey(); // Send to self

    println!("Sending 1 lamport from {} to {}", payer.pubkey(), to_pubkey);

    let balance = wallet::print_balance(&rpc_client, &payer.pubkey())?;

    if balance < 5000 + 1 {
        anyhow::bail!("Not enough SOL to cover transaction fee + 1 lamport");
    }

    let instruction = system_instruction::transfer(&payer.pubkey(), &to_pubkey, 1);
    let recent_blockhash = rpc_client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction confirmed!");
    println!("Signature: {}", signature);
    println!("Explorer: https://explorer.solana.com/tx/{}", signature);

    Ok(())
}
