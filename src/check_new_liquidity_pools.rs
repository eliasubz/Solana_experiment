use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::commitment_config::CommitmentConfig;
// use solana_sdk::pubkey::Pubkey;
// use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use regex::Regex;
// use std::collections::HashSet;
// use std::str::FromStr;

pub fn check_new_liquidity_pools(
    slot: u64,
    adress: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Manually construct the RPC request to include maxSupportedTransactionVersion = 0
    let params = json!([slot, {
        "encoding": "json",
        "transactionDetails": "full",
        "rewards": false,
        "maxSupportedTransactionVersion": 0 // This is the key part to support version 0 transactions
    }]);

    // Correct the send method to use RpcRequest::GetBlock and pass the parameters
    let block: serde_json::Value = rpc_client.send(RpcRequest::GetBlock, params)?;

    // Define the program IDs of known DEXs (e.g., Serum, Raydium)
    let dex_program_id = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"; // Raydium liquidity mint

    // let dexscreener_base_url = "https://dexscreener.com/solana/";

    // Parse the block and iterate over transactions
    if let Some(transactions) = block["transactions"].as_array() {
        for transaction_with_meta in transactions {
            // println!("Transaction: {}", transaction_with_meta);
            // let target_address = "djgmdpnu9nagjnpx96dvdjfczrkyem3ulq67amp4rnhj";
            if search_address_in_transaction(transaction_with_meta, adress)
                || search_address_in_transaction(
                    transaction_with_meta,
                    "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C",
                )
            {
                if contains_initialize_mint(transaction_with_meta) {
                    println!("\nNew transaction detected in block: ");
                    // println!("Transaction: {}", transaction_with_meta);
                    return Ok(transaction_with_meta.clone());
                }
            }
        }
    }

    return Ok(json!({}));
}

fn search_address_in_transaction(transaction: &Value, target_address: &str) -> bool {
    let mut return_value = false;

    // regex to find the address in the data field
    let transaction_str = transaction.to_string();

    // Create a regular expression to search for the target address
    let re = Regex::new(target_address).unwrap();

    // Use the regex to search for the address in the stringified JSON
    return_value = re.is_match(&transaction_str);

    if !is_err_null(transaction) {
        return_value = false;
    }
    // Address not found in this transaction
    return return_value;
}

fn is_err_null(transaction: &Value) -> bool {
    if let Some(meta) = transaction.get("meta") {
        if let Some(err) = meta.get("err") {
            return err.is_null();
        }
    }
    false
}

fn contains_initialize_mint(transaction: &Value) -> bool {
    if let Some(log_messages) = transaction["meta"].get("logMessages") {
        if let Some(logs) = log_messages.as_array() {
            for log in logs {
                if let Some(log_str) = log.as_str() {
                    if log_str.contains("InitializeMint") {
                        return true;
                    }
                }
            }
        }
    }
    false
}
