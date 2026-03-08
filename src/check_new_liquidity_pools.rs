use rayon::prelude::*;
use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::commitment_config::CommitmentConfig;

const RAYDIUM_AMM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RAYDIUM_CPMM: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";

/// Scan a block for new Raydium liquidity pool creations.
/// Returns the first matching transaction, or an empty JSON object if none found.
pub fn check_new_liquidity_pools(
    slot: u64,
    address: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let params = json!([slot, {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false,
        "maxSupportedTransactionVersion": 0
    }]);

    let block: Value = rpc_client.send(RpcRequest::GetBlock, params)?;

    let targets = [address, RAYDIUM_AMM, RAYDIUM_CPMM];

    if let Some(transactions) = block["transactions"].as_array() {
        // Use rayon to scan transactions in parallel across CPU cores
        let result = transactions
            .par_iter()
            .find_any(|tx| is_new_liquidity_pool(tx, &targets))
            .cloned();

        if let Some(tx) = result {
            println!("\nNew liquidity pool detected in slot {}", slot);
            return Ok(tx);
        }
    }

    Ok(json!({}))
}

/// Scan a block and return ALL matching liquidity pool transactions.
pub fn find_all_new_liquidity_pools(
    slot: u64,
    address: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let params = json!([slot, {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false,
        "maxSupportedTransactionVersion": 0
    }]);

    let block: Value = rpc_client.send(RpcRequest::GetBlock, params)?;

    let targets = [address, RAYDIUM_AMM, RAYDIUM_CPMM];

    let matches = block["transactions"]
        .as_array()
        .map(|txs| {
            txs.par_iter()
                .filter(|tx| is_new_liquidity_pool(tx, &targets))
                .cloned()
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if !matches.is_empty() {
        println!(
            "\nFound {} new liquidity pool(s) in slot {}",
            matches.len(),
            slot
        );
    }

    Ok(matches)
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
