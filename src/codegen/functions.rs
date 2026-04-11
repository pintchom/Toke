use super::emitter::Emitter;
use super::opcodes;

pub fn emit_total_supply(emitter: &mut Emitter) {
    emitter.emit_label("totalSupply");

    // SLOAD slot 0 → pushes totalSupply onto the stack
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SLOAD);

    // MSTORE(0x00, totalSupply) → write it to memory
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    // RETURN(0x00, 0x20) → return 32 bytes from memory
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);
}

pub fn emit_balance_of(emitter: &mut Emitter) {
    emitter.emit_label("balanceOf");

    // Read the address argument from calldata.
    // Calldata layout: [4-byte selector | 32-byte arg]
    // CALLDATALOAD(4) skips the selector and loads the address.
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x04);
    emitter.emit(opcodes::CALLDATALOAD);

    // Compute mapping slot: keccak256(abi.encode(address, 5))
    // Same approach as the constructor — lay out address and base slot
    // in memory, then hash.

    // MSTORE(0x00, address)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    // MSTORE(0x20, 5) — balanceOf base slot
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x05);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);

    // SHA3(0x00, 0x40)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);

    // SLOAD the balance from that slot
    emitter.emit(opcodes::SLOAD);

    // MSTORE(0x00, balance) → write to memory
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    // RETURN(0x00, 0x20) → return 32 bytes
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);
}
