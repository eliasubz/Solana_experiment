use anyhow::Result;
mod check_new_liquidity_pools;
mod find_initialize_in_block;
mod first_tx;
mod get_info;
mod launch_terminal;
mod recent_slots;
// mod rpc_client_get_block;
use check_new_liquidity_pools::check_new_liquidity_pools;
// use find_initialize_in_block::find_initialize_in_block;
use get_info::{extract_pool_keys, get_pair_key};
use recent_slots::get_recent_slots;
use serde_json::{json, Value};

fn main() -> Result<()> {
    // Example usage of calculate_send
    // calculate_send::calculate_sends(289921934);
    launch_terminal::launch_new_terminal();

    let mut liquidity_transaction_json: Result<serde_json::Value, Box<dyn std::error::Error>> =
        Ok(json!({}));

    /*
    // Get the transaction JSON
    liquidity_transaction_json = check_new_liquidity_pools(
        289921934,
        "3CApfZGRDowZHPMxJkw2N662Yd12kpD89ohxESkYMyCBBuGkgwtf6Pk24cc3pJqJ8MvEjVcKDbzHDTjTKRyH6ATR",
    );

    // Get pair key
    let pair_key = get_pair_key(&liquidity_transaction_json.unwrap()).unwrap();

    let dexscreener_base_url = "https://dexscreener.com/solana/";
    let pair_key_trimmed = pair_key.trim_matches('"');
    let token_url = format!("{}{}", dexscreener_base_url, pair_key_trimmed);
    println!("Link to Radium with this token: {}", token_url);
    */

    // Slots with initilaized Mints
    //let slot = [289715401, 289602670, 289733173, 289827292];
    // let slot = get_recent_slots()?;
    // println!("10 most recent slots: {:?}", slot);
    //let client = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    // Signatures of the Mintinizalization
    let target_signature = [
        "4f7xU4uWHonHMhiRAQ7J2Meq2xfFUsHXSMaC5vwhGm2YfSmduf6Ugb2Bot1LB2UmdV2gs1H4EUPAhC1e9Yg7SBvu",
        "63ghaciFGHkchhCw7Bdr3wY1pSzxQytNr6W4cpiJr5xGN4NsCANs57pDZAusuGWuijqfZVoMbqRSpgpx8emKQpx4",
        "2k4qM96n4uExfjV6tZoYRJpjJLRE2nM1N3trEetV3uMH1QRzX8xEQwFkcRfAV21Uppfj3qgi1C8RNZ63AKNDa8Jn",
    ];

    let addresses = [
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Radiyum liquidity pool
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C", // CPMM creation of token
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",  // InitializeMint2
    ];

    let slot = get_recent_slots()?;
    println!("{:?}", slot);
    //let slot = [289715401, 289602670, 289733173, 289827292];
    for i in 0..slot.len() {
        match check_new_liquidity_pools(slot[i], &addresses) {
            Ok(transaction) => {
                let pair_key = get_pair_key(&transaction);
                println!(
                    "Pair key: {}",
                    pair_key.unwrap_or_else(|| "Not found".into())
                );
            }
            Err(_) => todo!(),
        }
    }
    Ok(())
}
