use crate::ast::ContractNode;

use super::emitter::Emitter;
use super::opcodes;
use super::utils::selector;

pub fn emit_dispatcher(emitter: &mut Emitter, _contract: &ContractNode) {
    // Extract the 4-byte function selector from calldata.
    // CALLDATALOAD(0) loads 32 bytes starting at offset 0, then
    // SHR(224) right-shifts by 224 bits to isolate the top 4 bytes.
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::CALLDATALOAD); // loads 32 bytes starting at offset 0
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0xe0); // 224 bit shift (224 + 32  = 256)
    emitter.emit(opcodes::SHR);

    // Compare against each known selector and jump if matched.
    emit_selector_check(emitter, "totalSupply()", "totalSupply");
    emit_selector_check(emitter, "balanceOf(address)", "balanceOf");
    emit_selector_check(emitter, "transfer(address,uint256)", "transfer");

    // Fallback: no selector matched → revert
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::REVERT);
}

/// Emit a single selector comparison: DUP1, PUSH4 <selector>, EQ, PUSH2 <label>, JUMPI
fn emit_selector_check(emitter: &mut Emitter, signature: &str, label: &str) {
    let sel = selector(signature);

    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::PUSH4);
    emitter.emit_bytes(sel.as_slice());
    emitter.emit(opcodes::EQ);
    emitter.emit_jumpi_to(label);
}
