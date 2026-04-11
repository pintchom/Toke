use crate::ast::ContractNode;

use super::opcodes;
use super::utils::push_u64;

/// Emit constructor (deployment init) bytecode.
///
/// The constructor runs once during deployment. It:
/// 1. Stores totalSupply in storage slot 0
/// 2. Computes balanceOf[msg.sender] mapping slot and stores supply there
/// 3. Uses CODECOPY to copy the runtime bytecode into memory
/// 4. RETURNs the runtime bytecode (which becomes the deployed contract code)
///
/// `runtime_len` is the length of the runtime bytecode that follows the constructor.
pub fn emit_constructor(contract: &ContractNode, runtime_len: usize) -> Vec<u8> {
    let mut code = Vec::new();
    let supply = contract
        .supply
        .as_ref()
        .expect("supply validated by analyzer")
        .value;

    // start with recording total supply
    // Stack: [supply, 0] → SSTORE
    // pushing total supply value and slot 0 onto the stack, then SSTORE it
    // post reployment, anyone calling totalSupply() reads this slot
    push_u64(&mut code, supply);
    code.push(opcodes::PUSH1);
    code.push(0x00);
    code.push(opcodes::SSTORE);

    // then give tokens to the deployer
    // The mapping slot is keccak256(abi.encode(address, base_slot)).
    // base_slot for balanceOf is 5.
    //
    // We build the 64-byte preimage in memory:
    //   memory[0x00..0x20]  = msg.sender (left-padded to 32 bytes)
    //   memory[0x20..0x40]  = uint256(5)
    // Then SHA3 over those 64 bytes gives us the storage slot.

    code.push(opcodes::CALLER); // msg.sender (0x33)
    code.push(opcodes::PUSH1);
    code.push(0x00);
    code.push(opcodes::MSTORE);

    code.push(opcodes::PUSH1);
    code.push(0x05); // base slot number for balanceOf mapping 
    code.push(opcodes::PUSH1);
    code.push(0x20); // memory offset of 32 
    code.push(opcodes::MSTORE);

    // SHA3(0x00, 0x40) — hash 64 bytes to get the mapping slot
    code.push(opcodes::PUSH1);
    code.push(0x40); // length
    code.push(opcodes::PUSH1);
    code.push(0x00); // offset
    code.push(opcodes::SHA3);

    // SSTORE(mapping_slot, supply)
    push_u64(&mut code, supply);
    code.push(opcodes::SWAP1);
    code.push(opcodes::SSTORE);

    // copy runtime output into memory
    // CODECOPY(destOffset, offset, size)
    //   destOffset = 0x00 (write to start of memory)
    //   offset     = constructor_len (runtime starts right after constructor)
    //   size       = runtime_len
    //
    // We don't know constructor_len yet — it's the length of `code` after
    // we finish emitting, including these CODECOPY + RETURN instructions.
    // So we emit a placeholder and patch it below.
    push_u64(&mut code, runtime_len as u64); // size
    code.push(opcodes::PUSH2); // this is a placeholder thats patched later, so we dont optimistically set incorrect offset 
    let offset_patch_pos = code.len();
    code.push(0x00); // placeholder for constructor_len (high byte)
    code.push(0x00); // placeholder for constructor_len (low byte)
    code.push(opcodes::PUSH1);
    code.push(0x00); // destOffset
    code.push(opcodes::CODECOPY);

    push_u64(&mut code, runtime_len as u64); // size
    code.push(opcodes::PUSH1);
    code.push(0x00); // offset
    code.push(opcodes::RETURN);

    let constructor_len = code.len();
    let len_bytes = (constructor_len as u16).to_be_bytes();
    code[offset_patch_pos] = len_bytes[0];
    code[offset_patch_pos + 1] = len_bytes[1];

    code
}
