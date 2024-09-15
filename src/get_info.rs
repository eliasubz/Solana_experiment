extern crate serde_json;
use serde_json::{Number, Value};
use solana_sdk::address_lookup_table::instruction;

pub fn get_pair_key(json: &Value) -> Option<String> {
    // Find accountkeys
    let account_keys = &json["transaction"]["message"]["accountKeys"];
    let radyium_account_key_idx = get_radyium_account_key_idx(&account_keys);
    let rad_key_idx = radyium_account_key_idx.unwrap() as u64;

    let instructions = &json["transaction"]["message"]["instructions"];
    let mint_instruction = get_mint_instruction(&instructions, rad_key_idx).unwrap();
    print!("mint_instruction: {}", mint_instruction);

    // Find index of Keypair address
    let mut key_pair_idx: u64 = 0;
    if let Some(key_pair) = mint_instruction["accounts"].get(4) {
        println!("This is the key_pair: {}", key_pair);
        if let Some(key_pair_index_in_loop) = key_pair.as_u64() {
            key_pair_idx = key_pair_index_in_loop;
            println!("This is the key_pair: {}", key_pair);
        } else {
            println!("keypair wasnt an a u64 allegedly");
            return None;
        }
    }
    //
    let key_pair_idx = key_pair_idx as usize;
    let key_pair_address = get_key_id_with_index(account_keys, key_pair_idx).unwrap();

    println! {"key_pair_address: {}", key_pair_address};
    println!("This is after both of the if blocks.");

    // Return the 4th account as a String
    return Some(key_pair_address.to_string());
}

fn get_mint_instruction(instructions: &Value, rad_id: u64) -> Option<Value> {
    println!("{}", instructions);
    for i in 3..=5 {
        if let Some(instruction) = instructions.get(i) {
            println!("{}", instruction);
            if let Some(program_id) = instruction["programIdIndex"].as_u64() {
                println!("program_id: {}", &program_id);
                println!("rad_id: {}", &rad_id);
                println!("is it the same?: {}", program_id == rad_id);
                println!("wtf");
                let radium = bool::from(program_id == rad_id);
                print!("radium: {}", radium);
                if radium {
                    println!("apölfkjsd");
                    let instruction_copy = instruction.clone();
                    println!("instruction_copy: {}", instruction_copy);
                    return Some(instruction_copy);
                }
            }
        }
    }
    None
}

// Takes in the account_keys and the index of the key to find the index of the AMM Radium key
fn get_key_id_with_index(account_keys: &Value, key_index: usize) -> Option<Value> {
    println!("Looking for account address at index: {}", key_index);
    if let Some(key_address) = account_keys.get(key_index) {
        println!("Found the account address: {}", key_address);
        println!("This is the key_address: {}", account_keys);
        let key_address_clone = key_address.clone();
        return Some(key_address_clone);
    } else {
        println!("\n This is where Im looking through \n{}", account_keys);
        println!("Could not find the account address at index: {}", key_index);
    }

    None
}

// Takes in the account_keys and returns the index of the AMM Radium key
fn get_radyium_account_key_idx(account_keys: &Value) -> Option<usize> {
    println!("{}", account_keys);
    if let Some(account_keys_array) = account_keys.as_array() {
        for i in 0..account_keys_array.len() {
            if let Some(acc_key) = account_keys_array.get(i) {
                if acc_key == "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" {
                    println!(
                        "We found the idx of the raydium account key at index: {}",
                        i
                    );
                    return Some(i);
                }
            }
        }
    }
    None
}
