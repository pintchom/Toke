pub mod constructor;
pub mod dispatcher;
pub mod emitter;
pub mod functions;
pub mod opcodes;
pub mod utils;

use crate::ast::ContractNode;
use crate::errors::CompileError;

/// Generate deployment bytecode from a validated contract AST.
///
/// The output is a single byte vector containing both:
///   [constructor bytecode | runtime bytecode]
///
/// The constructor runs once during deployment, stores initial state,
/// then returns the runtime bytecode as the deployed contract code.
pub fn generate(contract: &ContractNode) -> Result<Vec<u8>, CompileError> {
    let _ = contract;
    todo!("Full bytecode generation pipeline")
}
