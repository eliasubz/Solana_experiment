use anyhow::Result;
use solana_client::rpc_client::RpcClient;

/// Returns the most recent confirmed slots.
pub fn get_recent_slots(rpc_client: &RpcClient) -> Result<Vec<u64>> {
    let current_slot = rpc_client.get_slot()?;

    // Return the last 10 confirmed slots
    let slots: Vec<u64> = (current_slot - 9..=current_slot).collect();

    Ok(slots)
}
