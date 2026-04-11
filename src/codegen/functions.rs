use crate::ast::ContractNode;

use super::emitter::Emitter;
use super::opcodes;
use super::utils::{event_topic, push_value};

const TRANSFER_TOPIC: &str = "Transfer(address,address,uint256)";

pub fn emit_name(emitter: &mut Emitter, contract: &ContractNode) {
    emitter.emit_label("name");
    emit_string_return(emitter, &contract.name);
}

pub fn emit_symbol(emitter: &mut Emitter, contract: &ContractNode) {
    emitter.emit_label("symbol");
    let symbol = match &contract.symbol {
        Some(s) => s.value.clone(),
        None => contract
            .name
            .chars()
            .take(3)
            .collect::<String>()
            .to_uppercase(),
    };
    emit_string_return(emitter, &symbol);
}

pub fn emit_decimals(emitter: &mut Emitter, contract: &ContractNode) {
    emitter.emit_label("decimals");
    let decimals = contract.decimals.as_ref().map(|d| d.value).unwrap_or(18);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(decimals as u8);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);
}

pub fn emit_approve(emitter: &mut Emitter) {
    emitter.emit_label("approve");

    // Load spender from calldata[0x04]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x04);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [spender]

    // Load amount from calldata[0x24]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x24);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [amount, spender]

    // Inner hash: keccak256(caller, 6)
    emitter.emit(opcodes::CALLER);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x06);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [inner_hash, amount, spender]

    // Outer hash: keccak256(spender, inner_hash)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE); // memory[0x20] = inner_hash
    // stack: [amount, spender]

    emitter.emit(opcodes::DUP2); // [spender, amount, spender]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = spender
    // stack: [amount, spender]

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [allowance_slot, amount, spender]

    // SSTORE: key=allowance_slot, value=amount
    emitter.emit(opcodes::SSTORE);
    // stack: [spender]
    emitter.emit(opcodes::POP);

    // Return true
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
}

pub fn emit_allowance(emitter: &mut Emitter) {
    emitter.emit_label("allowance");

    // Load owner from calldata[0x04]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x04);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [owner]

    // Load spender from calldata[0x24]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x24);
    emitter.emit(opcodes::CALLDATALOAD);
    // stack: [spender, owner]

    // Inner hash: keccak256(owner, 6)
    emitter.emit(opcodes::SWAP1); // [owner, spender]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = owner
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x06);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [inner_hash, spender]

    // Outer hash: keccak256(spender, inner_hash)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE); // memory[0x20] = inner_hash
    // stack: [spender]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = spender
    // stack: []

    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [allowance_slot]

    emitter.emit(opcodes::SLOAD);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);
}

pub fn emit_transfer_from(emitter: &mut Emitter) {
    emitter.emit_label("transferFrom");

    // Load args
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x44);
    emitter.emit(opcodes::CALLDATALOAD); // amount
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x24);
    emitter.emit(opcodes::CALLDATALOAD); // to
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x04);
    emitter.emit(opcodes::CALLDATALOAD); // from
    // stack: [from, to, amount]

    // Check and deduct allowance[from][caller] ---
    // Inner hash: keccak256(from, 6)
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x06);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [inner_hash, from, to, amount]

    // Outer hash: keccak256(caller, inner_hash)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE); // memory[0x20] = inner_hash
    emitter.emit(opcodes::CALLER);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = caller
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::SHA3);
    // stack: [allowance_slot, from, to, amount]

    // Load allowance, check >= amount
    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::SLOAD);
    // stack: [allowance_val, allowance_slot, from, to, amount]

    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::DUP6); // amount
    emitter.emit(opcodes::GT);
    emitter.emit_jumpi_to("revert_transferFrom");
    // stack: [allowance_val, allowance_slot, from, to, amount]

    // Deduct: new_allowance = allowance_val - amount
    emitter.emit(opcodes::DUP5); // amount
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SUB);
    // stack: [new_allowance, allowance_slot, from, to, amount]
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SSTORE);
    // stack: [from, to, amount]

    // Deduct from sender's balance ---
    // keccak256(from, 5)
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
    // stack: [from_slot, from, to, amount]

    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::SLOAD);
    // stack: [from_bal, from_slot, from, to, amount]

    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::DUP6); // amount
    emitter.emit(opcodes::GT);
    emitter.emit_jumpi_to("revert_transferFrom");
    // stack: [from_bal, from_slot, from, to, amount]

    emitter.emit(opcodes::DUP5); // amount
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SUB);
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SSTORE);
    // stack: [from, to, amount]

    // Credit recipient's balance ---
    // keccak256(to, 5)
    emitter.emit(opcodes::SWAP1); // [to, from, amount]
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
    // stack: [to_slot, to, from, amount]

    emitter.emit(opcodes::DUP1);
    emitter.emit(opcodes::SLOAD);
    emitter.emit(opcodes::DUP5); // amount
    emitter.emit(opcodes::ADD);
    emitter.emit(opcodes::SWAP1);
    emitter.emit(opcodes::SSTORE);
    // stack: [to, from, amount]

    // Emit Transfer(from, to, amount) event
    // stack: [to, from, amount]
    emitter.emit(opcodes::DUP3); // [amount, to, from, amount]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = amount

    // LOG3: push topic2, topic1, topic0, size, offset
    emitter.emit(opcodes::DUP1); // [to, to, from, amount] — topic2
    emitter.emit(opcodes::DUP3); // [from, to, to, from, amount] — topic1
    let topic = event_topic(TRANSFER_TOPIC);
    emitter.emit(opcodes::PUSH32);
    emitter.emit_bytes(&topic); // topic0
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20); // size
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00); // offset
    emitter.emit(opcodes::LOG3);
    // stack: [to, from, amount]

    emitter.emit(opcodes::POP);
    emitter.emit(opcodes::POP);
    emitter.emit(opcodes::POP);

    // Return true
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

    emitter.emit_label("revert_transferFrom");
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::REVERT);
}

fn emit_string_return(emitter: &mut Emitter, s: &str) {
    // MSTORE(0x00, 0x20) — offset pointer
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE);

    // MSTORE(0x20, string length)
    emitter.emit(opcodes::PUSH1);
    emitter.emit(s.len() as u8);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20);
    emitter.emit(opcodes::MSTORE);

    // MSTORE(0x40, string data right-padded to 32 bytes)
    let mut padded = [0u8; 32];
    let bytes = s.as_bytes();
    padded[..bytes.len().min(32)].copy_from_slice(&bytes[..bytes.len().min(32)]);
    push_value(&mut Vec::new(), &padded); // just to validate
    emitter.emit(opcodes::PUSH32);
    emitter.emit_bytes(&padded);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x40);
    emitter.emit(opcodes::MSTORE);

    // RETURN(0x00, 0x60) — return 96 bytes
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x60);
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::RETURN);
}

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

    // Emit Transfer(caller, to, amount) event
    // stack: [to, amount]
    emitter.emit(opcodes::DUP2); // [amount, to, amount]
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00);
    emitter.emit(opcodes::MSTORE); // memory[0x00] = amount

    // LOG3: push topic2, topic1, topic0, size, offset
    emitter.emit(opcodes::DUP1); // [to, to, amount] — topic2
    emitter.emit(opcodes::CALLER); // [caller, to, to, amount] — topic1
    let topic = event_topic(TRANSFER_TOPIC);
    emitter.emit(opcodes::PUSH32);
    emitter.emit_bytes(&topic); // topic0
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x20); // size
    emitter.emit(opcodes::PUSH1);
    emitter.emit(0x00); // offset
    emitter.emit(opcodes::LOG3);
    // stack: [to, amount]

    emitter.emit(opcodes::POP);
    emitter.emit(opcodes::POP);

    // Return true
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
