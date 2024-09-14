use solana_transaction_status::EncodedConfirmedBlock;

pub fn get_instructions(block: EncodedConfirmedBlock) -> String {
    let mut instructions = String::new();
    for tx in block.transactions {
        for message in tx.message.instructions {
            instructions.push_str(&format!("{:?}\n", message));
        }
    }
    return instructions;
}
