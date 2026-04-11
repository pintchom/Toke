pub mod constructor;
pub mod dispatcher;
pub mod emitter;
pub mod functions;
pub mod opcodes;
pub mod utils;

use crate::ast::ContractNode;
use crate::errors::CompileError;

pub fn generate(contract: &ContractNode) -> Result<Vec<u8>, CompileError> {
    // build the runtime bytecode (dispatcher + function handlers)
    let mut emitter = emitter::Emitter::new();
    dispatcher::emit_dispatcher(&mut emitter, contract);
    functions::emit_name(&mut emitter, contract);
    functions::emit_symbol(&mut emitter, contract);
    functions::emit_decimals(&mut emitter, contract);
    functions::emit_total_supply(&mut emitter);
    functions::emit_balance_of(&mut emitter);
    functions::emit_transfer(&mut emitter);
    functions::emit_approve(&mut emitter);
    functions::emit_allowance(&mut emitter);
    functions::emit_transfer_from(&mut emitter);

    // patch all forward jump placeholders with real offsets
    emitter.resolve()?;
    let runtime = emitter.into_bytes();

    // build the constructor, telling it how long the runtime is
    let constructor = constructor::emit_constructor(contract, runtime.len());

    // concatenate: [constructor | runtime]
    let mut bytecode = constructor;
    bytecode.extend_from_slice(&runtime);
    Ok(bytecode)
}
