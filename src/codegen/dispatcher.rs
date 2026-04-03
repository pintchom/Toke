use crate::ast::ContractNode;

use super::emitter::Emitter;

/// Emit the function dispatcher into the emitter.
///
/// The dispatcher is the entry point of the runtime bytecode. It:
/// 1. Reads the first 4 bytes of calldata (the function selector)
/// 2. Compares against each known selector (totalSupply, balanceOf, etc.)
/// 3. Jumps to the matching handler label
/// 4. If no selector matches, falls through to REVERT (fallback)
pub fn emit_dispatcher(emitter: &mut Emitter, contract: &ContractNode) {
    let _ = (emitter, contract);
    todo!("Function dispatcher generation")
}
