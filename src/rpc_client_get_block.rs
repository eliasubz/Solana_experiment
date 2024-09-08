// Todo:
// - turn Block into Result<Block, Error> and handle errors
// - how to return the block
// - CHECK

use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::EncodedConfirmedBlock;

pub fn get_block(slot: u64) -> Result<EncodedConfirmedBlock, ClientError> {
    // Initialize the RPC client
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Specify the slot number you want to query

    // Get the block
    let block = rpc_client.get_block(slot);
    // Ok(block) => {
    //     // println!("{:#?}", block);
    //     // Print size
    //     println!("Block for slot {}:", slot);
    //     println!(
    //         "Number of transactions in slot: {}",
    //         block.transactions.len()
    //     );
    //     println!("Block hash: {}", block.blockhash);
    //     println!(
    //         "Block size: {} bytes",
    //         bincode::serialized_size(&block).unwrap()
    //     ); // Question Mark if we return reuslt
    //     println!("Parent slot: {}", block.parent_slot);
    //     println!("Block time: {}", block.block_time.unwrap());
    //     println!("Block height: {}", block.block_height.unwrap());
    //     // let first_rewards = [..5].to_vec();
    //     println!("reward: {:?}", block.rewards);
    let first_transactions = block.as_ref().unwrap().transactions[..3].to_vec();
    println!("First 5 transactions: {:?}", first_transactions);
    // }
    // Err(e) => {
    //     eprintln!("Error fetching block: {}", e);
    // }
    // };

    return block;
}
