use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use anyhow::Result;

pub fn get_recent_slots() -> Result<Vec<u64>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Fetch the most recent slot
    let current_slot: u64 = rpc_client.get_slot()?;

    // Get the 10 most recent slots
    let recent_slots: Vec<u64> = (current_slot - 99..=current_slot).collect();

    // Return the vector of recent slots
    Ok(recent_slots)
}