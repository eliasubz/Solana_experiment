extern crate serde_json;
use serde_json::Value;

const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

/// All the key addresses extracted from a Raydium AMM initialize instruction.
/// These are the accounts needed to interact with the pool (e.g. swap).
#[derive(Debug, Clone)]
pub struct RaydiumPoolKeys {
    pub amm_id: String,
    pub amm_authority: String,
    pub amm_open_orders: String,
    pub lp_mint: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub base_vault: String,
    pub quote_vault: String,
    pub target_orders: String,
    pub serum_program: String,
    pub serum_market: String,
}

/// Extract all pool keys from a Raydium AMM initialize transaction.
pub fn extract_pool_keys(transaction: &Value) -> Option<RaydiumPoolKeys> {
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    let raydium_idx = find_program_index(account_keys, RAYDIUM_AMM_PROGRAM)? as u64;

    let instructions = &transaction["transaction"]["message"]["instructions"];
    let ix = find_raydium_instruction(instructions, raydium_idx)?;

    let accs = ix["accounts"].as_array()?;

    // Raydium AMM V4 initialize2 account layout:
    //  0  token_program
    //  1  spl_associated_token_account
    //  2  system_program
    //  3  rent
    //  4  amm_id
    //  5  amm_authority
    //  6  amm_open_orders
    //  7  lp_mint
    //  8  base_mint (coin)
    //  9  quote_mint (pc, usually WSOL)
    // 10  base_vault (pool coin token account)
    // 11  quote_vault (pool pc token account)
    // 12  pool_withdraw_queue (unused in newer versions)
    // 13  target_orders
    // 14  pool_lp_token_account
    // 15  pool_temp_lp_token_account
    // 16  serum_program
    // 17  serum_market
    let resolve = |idx: usize| -> Option<String> {
        let key_idx = accs.get(idx)?.as_u64()? as usize;
        Some(account_keys.get(key_idx)?.as_str()?.to_string())
    };

    Some(RaydiumPoolKeys {
        amm_id: resolve(4)?,
        amm_authority: resolve(5)?,
        amm_open_orders: resolve(6)?,
        lp_mint: resolve(7)?,
        base_mint: resolve(8)?,
        quote_mint: resolve(9)?,
        base_vault: resolve(10)?,
        quote_vault: resolve(11)?,
        target_orders: resolve(13)?,
        serum_program: resolve(16)?,
        serum_market: resolve(17)?,
    })
}

/// Convenience wrapper — returns just the AMM pair key (kept for backwards compat).
pub fn get_pair_key(json: &Value) -> Option<String> {
    extract_pool_keys(json).map(|keys| keys.amm_id)
}

/// Find the Raydium instruction inside the transaction's instructions array.
fn find_raydium_instruction(instructions: &Value, raydium_program_idx: u64) -> Option<Value> {
    for instruction in instructions.as_array()? {
        if instruction["programIdIndex"].as_u64() == Some(raydium_program_idx) {
            return Some(instruction.clone());
        }
    }
    None
}

/// Find the index of a program ID in the accountKeys array.
fn find_program_index(account_keys: &Value, program_id: &str) -> Option<usize> {
    account_keys
        .as_array()?
        .iter()
        .position(|key| key.as_str() == Some(program_id))
}
