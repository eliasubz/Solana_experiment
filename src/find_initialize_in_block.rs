use serde_json::{json, Value};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_request::RpcRequest;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
// use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

pub fn find_initialize_in_block(slot: u64, adress: &str) -> Result<(), Box<dyn std::error::Error>> {
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
    let dex_program_ids: HashSet<Pubkey> = vec![
        //"4k3Dyjzvzp8e19MMm3QCrw8u3o5YZZczoEtnGV7CjLRc", // Raydium Liquidity Pool Program ID
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Raydium liquidity mint
        "9xQeWvG816bUx9EPB6XSzRkmPG1mT7J5t5g7PptHhDPH", // Serum DEX Program ID
    ]
    .into_iter()
    .map(|s| Pubkey::from_str(s).unwrap())
    .collect();

    // let dexscreener_base_url = "https://dexscreener.com/solana/";

    // Parse the block and iterate over transactions
    if let Some(transactions) = block["transactions"].as_array() {
        for transaction_with_meta in transactions {
            // println!("Transaction: {}", transaction_with_meta);
            // let target_address = "djgmdpnu9nagjnpx96dvdjfczrkyem3ulq67amp4rnhj";
            if search_address_in_transaction(transaction_with_meta, adress) {
                println!("\nNew transaction detected in block: ");
                println!("Transaction: {}", transaction_with_meta);
            }
            // let transaction = &transaction_with_meta["transaction"];
            // // println!("{}", transaction);

            // // Decode the transaction (you might need to adjust how you handle this based on the returned JSON structure)
            // let message = &transaction["message"];
            // let instructions = message["instructions"]
            //     .as_array()
            //     .unwrap_or(&vec![]) // Return a reference to an empty vector if None
            //     .to_owned(); // Clone the vector to avoid the reference issue

            // for instruction in instructions {
            //     let program_id_index = instruction["programIdIndex"].as_u64().unwrap() as usize;
            //     let account_keys = message["accountKeys"].as_array().unwrap();
            //     let program_id_str = account_keys[program_id_index].as_str().unwrap();

            //     // Ensure the program_id is valid and parse it into Pubkey
            //     match Pubkey::from_str(program_id_str) {
            //         Ok(program_id) => {
            //             //println!("Parsed Program ID: {}", program_id);  // Debugging print

            //             // Check if the program ID matches a known DEX liquidity pool program
            //             if dex_program_ids.contains(&program_id) {
            //                 // let token_url = format!("{}{}", dexscreener_base_url, program_id);
            //                 // println!(
            //                 //     "New liquidity pool creation detected in transaction: {:?}",
            //                 //     transaction
            //                 // );
            //                 // println!("Link to Radium with this token: {}", token_url);
            //             } else {
            //                 //println!("Program ID did not match known DEX IDs");  // Debugging print
            //             }
            //         }
            //         Err(_) => {
            //             eprintln!("Failed to parse program ID: {}", program_id_str);
            //             // Debugging print
            //         }
            //     }
            // }
        }
    }

    Ok(())
}

fn search_address_in_transaction(transaction: &Value, target_address: &str) -> bool {
    let mut return_value = false;
    // Check if the "accountKeys" array contains the target address
    if let Some(account_keys) = transaction.get("accountKeys") {
        if account_keys.is_array() {
            for key in account_keys.as_array().unwrap() {
                if key == target_address {
                    print!("Found address in account keys");
                    return_value = true;
                }
            }
        }
    }

    // Check outer instructions for accounts and data
    if let Some(instructions) = transaction.get("instructions") {
        if instructions.is_array() {
            for instruction in instructions.as_array().unwrap() {
                if let Some(accounts) = instruction.get("accounts") {
                    if accounts.is_array() {
                        for account in accounts.as_array().unwrap() {
                            if account == target_address {
                                print!("Found address in outer instructions");
                                return_value = true;
                            }
                        }
                    }
                }
            }
        }
    }

    // Check inner instructions (nested within outer transaction)
    if let Some(meta) = transaction.get("meta") {
        if let Some(inner_instructions) = meta.get("innerInstructions") {
            if inner_instructions.is_array() {
                for instruction in inner_instructions.as_array().unwrap() {
                    if let Some(accounts) = instruction.get("accounts") {
                        if accounts.is_array() {
                            for account in accounts.as_array().unwrap() {
                                if account == target_address {
                                    print!("Found address in inner instructions");
                                    return_value = true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // regex to find the address in the data field
    let transaction_str = transaction.to_string();

    // Create a regular expression to search for the target address
    let re = Regex::new(target_address).unwrap();

    // Use the regex to search for the address in the stringified JSON
    return_value = re.is_match(&transaction_str);

    // Address not found in this transaction
    return return_value;
}
