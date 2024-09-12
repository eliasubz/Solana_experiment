use anyhow::Result;
mod launch_terminal;
mod rpc_client_get_block;
mod monitor_program;
mod check_new_liquidity_pools;
use check_new_liquidity_pools::check_new_liquidity_pools;

fn main() -> Result<()> {
    // Terminal testing
    launch_terminal::launch_new_terminal()?;

    // Specify the slot number you want to query
    //let slot = 15100000; // Replace with the desired slot number
    //let _ = rpc_client_get_block::get_block(slot);

    //let client = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    //monitor_program::monitor_program(&client)?; // Call the new function

    let slot = 200000000; // Example slot number
    match check_new_liquidity_pools(slot) {
        Ok(_) => println!("Check completed successfully."),
        Err(err) => eprintln!("Error occurred: {:?}", err),
    }

    Ok(())
}
