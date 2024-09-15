extern crate serde_json;
use serde_json::Value;

pub fn get_pair_key(json: &Value) -> Option<String> {
    // Navigate to the 4th account in the "accounts" array
    let fourth_account = json["transaction"]["message"]["instructions"].get(4)?["accounts"]
        .get(3)?
        .as_str()?;

    // We want to find the program ID of the 4th instruction
    let program_id =
        json["transaction"]["message"]["instructions"].get(4)?["programId"].as_str()?;

    // Check if the program ID is the same as the one we are looking for
    if program_id == "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" {
        println!("This instruction talks to the Raydium Liquidity Pool program/account!");
    }

    // Return the 4th account as a String
    Some(fourth_account.to_string())
}
