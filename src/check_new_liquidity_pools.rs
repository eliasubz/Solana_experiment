use rayon::prelude::*;
use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use std::collections::HashMap;

/// Scan a block for new liquidity pool creations.
/// Returns the first matching (transaction, matched_address, label), or None.
pub fn check_new_liquidity_pools(
    rpc_client: &RpcClient,
    slot: u64,
    addresses: &HashMap<&str, &str>,
) -> Result<Option<(Value, String, String)>, Box<dyn std::error::Error>> {
    let keys: Vec<&str> = addresses.keys().copied().collect();

    let params = json!([slot, {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false,
        "maxSupportedTransactionVersion": 0
    }]);

    let block: Value = rpc_client.send(RpcRequest::GetBlock, params)?;

    if let Some(transactions) = block["transactions"].as_array() {
        let result = transactions.par_iter().find_map_any(|tx| {
            if !is_successful(tx) {
                return None;
            }
            let matched = find_matched_address(tx, &keys)?;
            if !has_initialize_mint_log(tx) {
                return None;
            }
            let label = addresses[matched].to_string();
            Some((tx.clone(), matched.to_string(), label))
        });

        if let Some((tx, addr, label)) = result {
            println!("\nNew liquidity pool detected in slot {}", slot);
            return Ok(Some((tx, addr, label)));
        }
    }

    Ok(None)
}

/// Check meta.err is null (transaction succeeded).
fn is_successful(transaction: &Value) -> bool {
    transaction
        .get("meta")
        .and_then(|meta| meta.get("err"))
        .map_or(false, |err| err.is_null())
}

/// Find which target address appears in this transaction's accountKeys.
/// Returns the matched address string, or None if no match.
fn find_matched_address<'a>(transaction: &Value, targets: &[&'a str]) -> Option<&'a str> {
    // Check static accountKeys in the message
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    if let Some(keys) = account_keys.as_array() {
        for key in keys {
            if let Some(key_str) = key.as_str() {
                if let Some(&matched) = targets.iter().find(|&&t| t == key_str) {
                    return Some(matched);
                }
            }
        }
    }

    // Also check loadedAddresses for v0 transactions
    if let Some(loaded) = transaction
        .get("meta")
        .and_then(|m| m.get("loadedAddresses"))
    {
        for section in &["writable", "readonly"] {
            if let Some(addrs) = loaded.get(section).and_then(|v| v.as_array()) {
                for addr in addrs {
                    if let Some(addr_str) = addr.as_str() {
                        if let Some(&matched) = targets.iter().find(|&&t| t == addr_str) {
                            return Some(matched);
                        }
                    }
                }
            }
        }
    }

    None
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
