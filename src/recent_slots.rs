use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

/// Returns the most recent confirmed slot.
pub fn get_recent_slots() -> Result<Vec<u64>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let current_slot = rpc_client.get_slot()?;

    // Return the last 100 confirmed slots
    let slots: Vec<u64> = (current_slot - 99..=current_slot).collect();

    Ok(slots)
}
