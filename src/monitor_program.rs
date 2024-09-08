use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use std::str::FromStr;

const RAYDIUM_PUBLIC_KEY: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

pub fn monitor_program(client: &RpcClient) -> Result<()> {
    let program_address = Pubkey::from_str(RAYDIUM_PUBLIC_KEY)?;
    println!("Monitoring logs for program: {:?}", program_address);

    let signatures = client.get_signatures_for_address(&program_address)?;

    for signature_info in signatures {
        let signature = signature_info.signature;
        println!("Found signature: https://explorer.solana.com/tx/{}", signature);
    }

    Ok(())
}
