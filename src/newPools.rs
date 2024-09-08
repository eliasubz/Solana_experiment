use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcTransactionConfig,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Signature,
};
use std::str::FromStr;

const RAYDIUM_PUBLIC_KEY: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const HTTP_URL: &str = "https://api.mainnet-beta.solana.com";
const INSTRUCTION_NAME: &str = "initialize2";

fn start_connection(client: &RpcClient, program_address: Pubkey) {
    println!("Monitoring logs for program: {:?}", program_address);

    match client.get_signatures_for_address(&program_address) {
        Ok(signatures) => {
            for signature_info in signatures {
                let signature = signature_info.signature;
                println!("Found signature: https://explorer.solana.com/tx/{}", signature);
                fetch_raydium_mints(client, signature.to_string());
            }
        }
        Err(e) => eprintln!("Error fetching signatures: {}", e),
    }
}

fn fetch_raydium_mints(client: &RpcClient, tx_id: String) {
    let signature = Signature::from_str(&tx_id).unwrap();
    let config = RpcTransactionConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    match client.get_transaction_with_config(&signature, config) {
        Ok(Some(tx)) => {
            let instructions = &tx.transaction.message.instructions;
            for ix in instructions {
                if ix.program_id == Pubkey::from_str(RAYDIUM_PUBLIC_KEY).unwrap() {
                    let accounts = &ix.accounts;
                    if let (Some(token_a_account), Some(token_b_account)) = (accounts.get(8), accounts.get(9)) {
                        println!("New LP Found");
                        println!("Token A: {:?}", token_a_account);
                        println!("Token B: {:?}", token_b_account);
                    } else {
                        println!("Could not find token accounts in transaction.");
                    }
                }
            }
        }
        Ok(None) => println!("No transaction found for ID: {}", tx_id),
        Err(e) => println!("Error fetching transaction: {}", e),
    }
}

fn main() {
    let client = RpcClient::new(HTTP_URL.to_string());
    let raydium_pubkey = Pubkey::from_str(RAYDIUM_PUBLIC_KEY).unwrap();

    start_connection(&client, raydium_pubkey);
}
