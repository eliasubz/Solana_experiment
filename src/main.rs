use anyhow::Result;
mod check_new_liquidity_pools;
mod find_initialize_in_block;
mod launch_terminal;
mod monitor_program;
mod rpc_client_get_block;
use check_new_liquidity_pools::check_new_liquidity_pools;
use find_initialize_in_block::find_initialize_in_block;

fn main() -> Result<()> {
    // Terminal testing
    launch_terminal::launch_new_terminal()?;

    // NEWW
    // Slots with initilaized Mints
    let slot = [289715401, 289602670, 289733173, 289827292];

    //let client = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    //monitor_program::monitor_program(&client)?; // Call the new function

    // Signatures of the Mintinizalization
    let target_signature = [
        "4f7xU4uWHonHMhiRAQ7J2Meq2xfFUsHXSMaC5vwhGm2YfSmduf6Ugb2Bot1LB2UmdV2gs1H4EUPAhC1e9Yg7SBvu",
        "63ghaciFGHkchhCw7Bdr3wY1pSzxQytNr6W4cpiJr5xGN4NsCANs57pDZAusuGWuijqfZVoMbqRSpgpx8emKQpx4",
        "2k4qM96n4uExfjV6tZoYRJpjJLRE2nM1N3trEetV3uMH1QRzX8xEQwFkcRfAV21Uppfj3qgi1C8RNZ63AKNDa8Jn",
        "3D86cvGHdVE4RkHfPjtmYZymu2RQVYVSL3ZNPPGJFTn5PoyVGwjs4ySCWtkZgpTqi8s121LRmfwDMrDVreR5JFo1",
    ];

    for i in 0..target_signature.len() {
        match find_initialize_in_block(slot[i], target_signature[i]) {
            Ok(_) => println!("Check completed successfully."),
            Err(err) => eprintln!("Error occurred: {:?}", err),
        }
    }

    // for i in slot..=slot {
    //     println!(); // Empty row
    //     println!(); // Empty row
    //     println!("Slot: {}", i);
    //     match check_new_liquidity_pools(slot) {
    //         Ok(_) => println!("Check completed successfully."),
    //         Err(err) => eprintln!("Error occurred: {:?}", err),
    //     }
    // }

    Ok(())
}
