use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::Keypair,
};

const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

/// Load the keypair from ~/.config/solana/id.json
pub fn load_keypair() -> Result<Keypair> {
    let keypair_path = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

    let keypair_bytes = std::fs::read_to_string(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair at {:?}: {}", keypair_path, e))?;

    let keypair_vec: Vec<u8> = serde_json::from_str(&keypair_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to parse keypair JSON: {}", e))?;

    let keypair = Keypair::from_bytes(&keypair_vec)
        .map_err(|e| anyhow::anyhow!("Invalid keypair bytes: {}", e))?;

    Ok(keypair)
}

/// Create a new RPC client connected to mainnet.
pub fn rpc_client(on_dev: Option<bool>) -> RpcClient {
    let url = if on_dev.unwrap_or(true){"https://api.devnet.solana.com"} else {RPC_URL};
    return RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());
}

/// Get and print the balance of the loaded wallet.
pub fn print_balance(client: &RpcClient, pubkey: &Pubkey) -> Result<u64> {
    let balance = client.get_balance(pubkey)?;
    println!(
        "Balance: {} SOL ({} lamports)",
        balance as f64 / LAMPORTS_PER_SOL as f64,
        balance
    );
    Ok(balance)
}
