use super::emitter::Emitter;

/// Emit the totalSupply() handler.
///
/// Loads storage slot 0 (where totalSupply lives), stores it in memory,
/// and returns 32 bytes.
pub fn emit_total_supply(emitter: &mut Emitter) {
    let _ = emitter;
    todo!("totalSupply handler")
}

/// Emit the balanceOf(address) handler.
///
/// Reads the address argument from calldata, computes the mapping storage
/// slot via keccak256(abi.encode(address, 5)), loads the balance, and
/// returns 32 bytes.
pub fn emit_balance_of(emitter: &mut Emitter) {
    let _ = emitter;
    todo!("balanceOf handler")
}
