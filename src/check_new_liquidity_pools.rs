use rayon::prelude::*;
use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;

/// Scan a block for new Raydium liquidity pool creations.
/// Returns the first matching transaction, or an empty JSON object if none found.
pub fn check_new_liquidity_pools(
    rpc_client: &RpcClient,
    slot: u64,
    addresses: &[&str],
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let params = json!([slot, {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false,
        "maxSupportedTransactionVersion": 0
    }]);

    let block: Value = rpc_client.send(RpcRequest::GetBlock, params)?;

    if let Some(transactions) = block["transactions"].as_array() {
        // Use rayon to scan transactions in parallel across CPU cores
        let result = transactions
            .par_iter()
            .find_any(|tx| is_new_liquidity_pool(tx, addresses))
            .cloned();

        if let Some(tx) = result {
            println!("\nNew liquidity pool detected in slot {}", slot);
            println!("\nThe transaction: {}", tx);
            println!("\nand address: {}", addresses[0]);
            return Ok(tx);
        }
    }

    Ok(json!({}))
}

/// Fast check: is this transaction a new liquidity pool creation?
/// Ordered from cheapest to most expensive check.
fn is_new_liquidity_pool(transaction: &Value, target_addresses: &[&str]) -> bool {
    // 1. Cheapest: skip failed transactions immediately
    if !is_successful(transaction) {
        return false;
    }

    // 2. Structural check: does accountKeys contain any target address?
    if !has_target_account_key(transaction, target_addresses) {
        return false;
    }

    // 3. Check logs for InitializeMint (new pool signal)
    has_initialize_mint_log(transaction)
}

/// Check meta.err is null (transaction succeeded).
fn is_successful(transaction: &Value) -> bool {
    transaction
        .get("meta")
        .and_then(|meta| meta.get("err"))
        .map_or(false, |err| err.is_null())
}

/// Structurally check accountKeys for any of the target addresses.
/// No serialization or regex — direct array comparison.
fn has_target_account_key(transaction: &Value, targets: &[&str]) -> bool {
    // Check static accountKeys in the message
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    if let Some(keys) = account_keys.as_array() {
        for key in keys {
            if let Some(key_str) = key.as_str() {
                if targets.contains(&key_str) {
                    return true;
                }
            }
        }
    }

    // Also check addressTableLookups and loadedAddresses for v0 transactions
    if let Some(loaded) = transaction
        .get("meta")
        .and_then(|m| m.get("loadedAddresses"))
    {
        for section in &["writable", "readonly"] {
            if let Some(addrs) = loaded.get(section).and_then(|v| v.as_array()) {
                for addr in addrs {
                    if let Some(addr_str) = addr.as_str() {
                        if targets.contains(&addr_str) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

/// Check logMessages for "InitializeMint" — the signal for new token mint / LP creation.
fn has_initialize_mint_log(transaction: &Value) -> bool {
    transaction
        .get("meta")
        .and_then(|meta| meta.get("logMessages"))
        .and_then(|logs| logs.as_array())
        .map_or(false, |logs| {
            logs.iter()
                .any(|log| log.as_str().map_or(false, |s| s.contains("InitializeMint")))
        })
}
