use crate::ast::ContractNode;

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
    let _ = (contract, runtime_len);
    todo!("Constructor bytecode generation")
}
