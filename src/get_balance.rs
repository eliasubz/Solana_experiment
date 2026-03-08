use anyhow::Result;
use crate::utils::wallet;
use solana_sdk::signature::Signer;

pub fn get_balance() -> Result<u64> {
    let client = wallet::rpc_client(true);
    let keypair = wallet::load_keypair()?;


    println!("Wallet address: {}", keypair.pubkey());
    let balance = wallet::print_balance(&client, &keypair.pubkey())?;

    Ok(balance)
}