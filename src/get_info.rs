extern crate serde_json;
use serde_json::Value;
use std::fmt;

const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RAYDIUM_CPMM_PROGRAM: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";

// ─── Pool key types per program ────────────────────────────────────

/// Raydium AMM V4 pool keys (initialize2 instruction).
#[derive(Debug, Clone)]
pub struct RaydiumAmmKeys {
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

/// Raydium CPMM pool keys (initialize instruction).
#[derive(Debug, Clone)]
pub struct CpmmKeys {
    pub creator: String,
    pub amm_config: String,
    pub authority: String,
    pub pool_state: String,
    pub token_0_mint: String,
    pub token_1_mint: String,
    pub lp_mint: String,
    pub token_0_vault: String,
    pub token_1_vault: String,
}

/// Token-2022 InitializeMint2 — just the new mint address.
#[derive(Debug, Clone)]
pub struct TokenMintInfo {
    pub mint: String,
}

/// Unified enum so callers can match on which program created the pool.
#[derive(Debug, Clone)]
pub enum PoolKeys {
    RaydiumAmm(RaydiumAmmKeys),
    Cpmm(CpmmKeys),
    TokenMint(TokenMintInfo),
}

// ─── Display impls ─────────────────────────────────────────────────

impl fmt::Display for RaydiumAmmKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔═══ Raydium AMM V4 Pool Keys ═══")?;
        writeln!(f, "║ AMM ID:        {}", self.amm_id)?;
        writeln!(f, "║ AMM Authority: {}", self.amm_authority)?;
        writeln!(f, "║ Open Orders:   {}", self.amm_open_orders)?;
        writeln!(f, "║ LP Mint:       {}", self.lp_mint)?;
        writeln!(f, "║ Base Mint:     {}", self.base_mint)?;
        writeln!(f, "║ Quote Mint:    {}", self.quote_mint)?;
        writeln!(f, "║ Base Vault:    {}", self.base_vault)?;
        writeln!(f, "║ Quote Vault:   {}", self.quote_vault)?;
        writeln!(f, "║ Target Orders: {}", self.target_orders)?;
        writeln!(f, "║ Serum Program: {}", self.serum_program)?;
        writeln!(f, "║ Serum Market:  {}", self.serum_market)?;
        write!(f, "╚═════════════════════════════════")
    }
}

impl fmt::Display for CpmmKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔═══ Raydium CPMM Pool Keys ═══")?;
        writeln!(f, "║ Creator:       {}", self.creator)?;
        writeln!(f, "║ AMM Config:    {}", self.amm_config)?;
        writeln!(f, "║ Authority:     {}", self.authority)?;
        writeln!(f, "║ Pool State:    {}", self.pool_state)?;
        writeln!(f, "║ Token 0 Mint:  {}", self.token_0_mint)?;
        writeln!(f, "║ Token 1 Mint:  {}", self.token_1_mint)?;
        writeln!(f, "║ LP Mint:       {}", self.lp_mint)?;
        writeln!(f, "║ Token 0 Vault: {}", self.token_0_vault)?;
        writeln!(f, "║ Token 1 Vault: {}", self.token_1_vault)?;
        write!(f, "╚═══════════════════════════════")
    }
}

impl fmt::Display for TokenMintInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔═══ Token-2022 Mint ═══")?;
        writeln!(f, "║ Mint: {}", self.mint)?;
        write!(f, "╚═══════════════════════")
    }
}

impl fmt::Display for PoolKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolKeys::RaydiumAmm(keys) => write!(f, "{}", keys),
            PoolKeys::Cpmm(keys) => write!(f, "{}", keys),
            PoolKeys::TokenMint(info) => write!(f, "{}", info),
        }
    }
}

// ─── Extraction logic ──────────────────────────────────────────────

/// Extract pool keys from a transaction, dispatching by matched program address.
pub fn extract_pool_keys(transaction: &Value, matched_address: &str) -> Option<PoolKeys> {
    match matched_address {
        RAYDIUM_AMM_PROGRAM => extract_raydium_amm(transaction).map(PoolKeys::RaydiumAmm),
        RAYDIUM_CPMM_PROGRAM => extract_cpmm(transaction).map(PoolKeys::Cpmm),
        _ => extract_token_mint(transaction).map(PoolKeys::TokenMint),
    }
}

/// Convenience wrapper — returns just the main pool/pair key regardless of program.
pub fn get_pair_key(transaction: &Value, matched_address: &str) -> Option<String> {
    extract_pool_keys(transaction, matched_address).map(|keys| match keys {
        PoolKeys::RaydiumAmm(k) => k.amm_id,
        PoolKeys::Cpmm(k) => k.pool_state,
        PoolKeys::TokenMint(k) => k.mint,
    })
}

// ─── Raydium AMM V4 ───────────────────────────────────────────────

fn extract_raydium_amm(transaction: &Value) -> Option<RaydiumAmmKeys> {
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    let program_idx = find_program_index(account_keys, RAYDIUM_AMM_PROGRAM)? as u64;
    let instructions = &transaction["transaction"]["message"]["instructions"];
    let ix = find_instruction_by_program(instructions, program_idx)?;
    let accs = ix["accounts"].as_array()?;

    // Raydium AMM V4 initialize2 account layout:
    //  0  token_program         4  amm_id          8  base_mint       12 withdraw_queue
    //  1  spl_ata               5  amm_authority   9  quote_mint      13 target_orders
    //  2  system_program        6  amm_open_orders 10 base_vault      16 serum_program
    //  3  rent                  7  lp_mint         11 quote_vault     17 serum_market
    let resolve = |idx: usize| -> Option<String> {
        let key_idx = accs.get(idx)?.as_u64()? as usize;
        Some(account_keys.get(key_idx)?.as_str()?.to_string())
    };

    Some(RaydiumAmmKeys {
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

// ─── Raydium CPMM ─────────────────────────────────────────────────

fn extract_cpmm(transaction: &Value) -> Option<CpmmKeys> {
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    let program_idx = find_program_index(account_keys, RAYDIUM_CPMM_PROGRAM)? as u64;
    let instructions = &transaction["transaction"]["message"]["instructions"];
    let ix = find_instruction_by_program(instructions, program_idx)?;
    let accs = ix["accounts"].as_array()?;

    // Raydium CPMM initialize account layout:
    //  0  creator               5  token_1_mint     10 token_1_vault
    //  1  amm_config            6  lp_mint          11 create_pool_fee
    //  2  authority             7  creator_token_0  12 observation_state
    //  3  pool_state            8  creator_token_1  13 token_program
    //  4  token_0_mint          9  token_0_vault    14+ system programs
    let resolve = |idx: usize| -> Option<String> {
        let key_idx = accs.get(idx)?.as_u64()? as usize;
        Some(account_keys.get(key_idx)?.as_str()?.to_string())
    };

    Some(CpmmKeys {
        creator: resolve(0)?,
        amm_config: resolve(1)?,
        authority: resolve(2)?,
        pool_state: resolve(3)?,
        token_0_mint: resolve(4)?,
        token_1_mint: resolve(5)?,
        lp_mint: resolve(6)?,
        token_0_vault: resolve(9)?,
        token_1_vault: resolve(10)?,
    })
}

// ─── Token-2022 InitializeMint2 ────────────────────────────────────

fn extract_token_mint(transaction: &Value) -> Option<TokenMintInfo> {
    // InitializeMint2 has the mint account as account index 0 in its instruction.
    // The Token-2022 program may appear multiple times; look for the instruction
    // whose data starts with the InitializeMint2 discriminator (20 = 0x14).
    let account_keys = &transaction["transaction"]["message"]["accountKeys"];
    let program_idx =
        find_program_index(account_keys, "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")? as u64;

    let instructions = transaction["transaction"]["message"]["instructions"].as_array()?;
    for ix in instructions {
        if ix["programIdIndex"].as_u64() != Some(program_idx) {
            continue;
        }
        // The mint account is the first account in InitializeMint2
        let accs = ix["accounts"].as_array()?;
        let key_idx = accs.first()?.as_u64()? as usize;
        let mint = account_keys.get(key_idx)?.as_str()?.to_string();
        return Some(TokenMintInfo { mint });
    }
    None
}

// ─── Helpers ───────────────────────────────────────────────────────

fn find_instruction_by_program(instructions: &Value, program_idx: u64) -> Option<Value> {
    for instruction in instructions.as_array()? {
        if instruction["programIdIndex"].as_u64() == Some(program_idx) {
            return Some(instruction.clone());
        }
    }
    None
}

fn find_program_index(account_keys: &Value, program_id: &str) -> Option<usize> {
    account_keys
        .as_array()?
        .iter()
        .position(|key| key.as_str() == Some(program_id))
}
