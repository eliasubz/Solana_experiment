use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransactionWithStatusMeta};
use serde_json::json;
use std::collections::HashSet;
use solana_sdk::pubkey::Pubkey;
use solana_client::rpc_request::RpcRequest;
use std::str::FromStr;

pub fn check_new_liquidity_pools(slot: u64) -> Result<(), Box<dyn std::error::Error>> {
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
    let block: serde_json::Value = rpc_client.send(
        RpcRequest::GetBlock,
        params,
    )?;

    // Define the program IDs of known DEXs (e.g., Serum, Raydium)
    let dex_program_ids: HashSet<Pubkey> = vec![
        //"4k3Dyjzvzp8e19MMm3QCrw8u3o5YZZczoEtnGV7CjLRc", // Raydium Liquidity Pool Program ID
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Raydium liquidity mint
        "9xQeWvG816bUx9EPB6XSzRkmPG1mT7J5t5g7PptHhDPH", // Serum DEX Program ID
    ]
    .into_iter()
    .map(|s| Pubkey::from_str(s).unwrap())
    .collect();

    let dexscreener_base_url = "https://dexscreener.com/solana/";

    // Parse the block and iterate over transactions
    if let Some(transactions) = block["transactions"].as_array() {
        for transaction_with_meta in transactions {

            
            let transaction = &transaction_with_meta["transaction"];
            // println!("{}", transaction);

            // Decode the transaction (you might need to adjust how you handle this based on the returned JSON structure)
            let message = &transaction["message"];
            let instructions = message["instructions"]
                .as_array()
                .unwrap_or(&vec![]) // Return a reference to an empty vector if None
                .to_owned(); // Clone the vector to avoid the reference issue

            for instruction in instructions {
                let program_id_index = instruction["programIdIndex"].as_u64().unwrap() as usize;
                let account_keys = message["accountKeys"].as_array().unwrap();
                let program_id_str = account_keys[program_id_index].as_str().unwrap();


                // Ensure the program_id is valid and parse it into Pubkey
                match Pubkey::from_str(program_id_str) {
                    Ok(program_id) => {
                        //println!("Parsed Program ID: {}", program_id);  // Debugging print

                        // Check if the program ID matches a known DEX liquidity pool program
                        if dex_program_ids.contains(&program_id) {
                            let token_url = format!("{}{}", dexscreener_base_url, program_id);
                            println!("New liquidity pool creation detected in transaction: {:?}", transaction);
                            println!("Link to Radium with this token: {}", token_url);
                        } else {
                            //println!("Program ID did not match known DEX IDs");  // Debugging print
                        }
                    },
                    Err(_) => {
                        eprintln!("Failed to parse program ID: {}", program_id_str);  // Debugging print
                    }
                }
            }
        }
    }

    Ok(())
}
