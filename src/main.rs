use anyhow::Result;
mod launch_terminal;
mod rpc_client_get_block;

fn main() -> Result<()> {
    // Terminal testing
    launch_terminal::launch_new_terminal()?;

    // Specify the slot number you want to query
    let slot = 15100000; // Replace with the desired slot number
    let _ = rpc_client_get_block::get_block(slot);

    Ok(())
}
