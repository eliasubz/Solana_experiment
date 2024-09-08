mod rpc_client_get_block;

fn calculate_sends(slot: u64) {
    let block = rpc_client_get_block::get_block(slot);
    block
}
