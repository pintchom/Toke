use super::emitter::Emitter;
use super::opcodes;

pub fn emit_transfer(emitter: &mut Emitter) {
    emitter.emit_label("transfer");

    // load amount from calldata[0x24]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x24);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [amount]

    // load recipient from calldata[0x04]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x04);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [to, amount]

    // compute sender's balance slot: keccak256(caller, 5) ---
    emitter.emit(opcodes::CALLER);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x05);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [sender_slot, to, amount]

    // load sender's balance
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::SLOAD);
    // stack: [sender_bal, sender_slot, to, amount]

    // check: if amount > sender_bal → revert (insufficient balance)
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::DUP5);
    emitter.emit(opcodes::GT);
    emitter.emit_jumpi_to("revert_transfer");
    // stack: [sender_bal, sender_slot, to, amount]

    // new_sender_bal = sender_bal - amount
    emitter.emit(opcodes::DUP4);
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SUB);
    // stack: [new_sender_bal, sender_slot, to, amount]

    // store new sender balance
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SSTORE);
    // stack: [to, amount]

    // compute recipient's balance slot: keccak256(to, 5) ---
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x05);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [recipient_slot, to, amount]

    // load recipient balance, add amount
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::SLOAD);
    emitter.emit(opcodes::DUP4);
    emitter.emit(opcodes::ADD);
    // stack: [new_recipient_bal, recipient_slot, to, amount]

    // store new recipient balance
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SSTORE);
    // stack: [to, amount]

    // clean up stack and return true (1)
    emitter.emit(opcodes::POP);
    emitter.emit(opcodes::POP);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x01);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);

    // revert path for insufficient balance
    emitter.emit_label("revert_transfer");
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::REVERT);
}

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
