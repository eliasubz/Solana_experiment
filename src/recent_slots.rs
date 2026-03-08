use anyhow::Result;
use solana_client::rpc_client::RpcClient;

/// Returns recent confirmed slots that actually have blocks.
/// Uses getBlocks RPC to skip missed/empty slots.
pub fn get_recent_slots(rpc_client: &RpcClient) -> Result<Vec<u64>> {
    let current_slot = rpc_client.get_slot()?;

    // Ask for blocks in a range; offset by 2 to avoid requesting slots
    // that are too fresh for the RPC to serve.
    let end = current_slot - 2;
    let start = end - 100;
    let slots = rpc_client.get_blocks(start, Some(end))?;

    Ok(slots)
}
