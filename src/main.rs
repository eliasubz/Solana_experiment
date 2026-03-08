use anyhow::Result;
mod check_new_liquidity_pools;
mod first_tx;
mod get_balance;
mod get_info;
mod recent_slots;
mod utils;

use check_new_liquidity_pools::check_new_liquidity_pools;
use first_tx::send_one_lamport;
use get_balance::get_balance;
use get_info::{extract_pool_keys, get_pair_key};
use recent_slots::get_recent_slots;
use crate::utils::wallet;

fn main() -> Result<()> {
    let rpc_client = wallet::rpc_client(Some(true));

    let balance = get_balance()?;
    println!("Balance: {} lamports", balance);

    send_one_lamport()?;

    let target_addresses = [
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8", // Raydium liquidity pool
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C", // CPMM creation of token
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",  // InitializeMint2
    ];

    let slots = get_recent_slots(&rpc_client)?;
    println!("{:?}", slots);

    for &slot in &slots {
        match check_new_liquidity_pools(&rpc_client, slot, &target_addresses) {
            Ok(transaction) => {
                let pair_key = get_pair_key(&transaction);
                let pool_keys = extract_pool_keys(&transaction);
                println!(
                    "Pair key: {}",
                    pair_key.unwrap_or_else(|| "Not found".into())
                );
                match pool_keys {
                    Some(keys) => println!("{}", keys),
                    None => println!("Pool keys: Not found"),
                }
            }
            Err(e) => eprintln!("Error scanning slot {}: {}", slot, e),
        }
    }
    Ok(())
}
