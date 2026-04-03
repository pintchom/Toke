use std::collections::HashMap;

use crate::errors::CompileError;

use super::opcodes;

/// A pending forward jump that needs to be patched once all labels are placed.
struct PendingJump {
    /// Byte position in `code` where the placeholder starts
    position: usize,
    /// Which label this jump targets
    target_label: String,
    /// How many bytes the placeholder occupies (matches the PUSH width)
    width: usize,
}

/// Bytecode builder with two-pass label/jump resolution.
///
/// The EVM's JUMP/JUMPI instructions require exact byte offsets as targets,
/// but when emitting a forward jump we don't yet know where the target label
/// will land. The Emitter solves this:
///
/// Pass 1: emit bytecode with placeholder zeros for jump targets, recording
///          each label's byte offset and each pending jump's location.
/// Pass 2: `resolve()` patches every placeholder with the real offset.
pub struct Emitter {
    code: Vec<u8>,
    labels: HashMap<String, usize>,
    pending_jumps: Vec<PendingJump>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }

    /// Append a single byte (typically an opcode).
    pub fn emit(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// Append a slice of bytes.
    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    /// Mark the current position as a named label and emit JUMPDEST.
    pub fn emit_label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.code.len());
        self.code.push(opcodes::JUMPDEST);
    }

    /// Emit PUSH2 <placeholder> JUMP targeting a named label.
    pub fn emit_jump_to(&mut self, label: &str) {
        self.code.push(opcodes::PUSH2);
        let pos = self.code.len();
        self.code.extend_from_slice(&[0x00, 0x00]);
        self.code.push(opcodes::JUMP);
        self.pending_jumps.push(PendingJump {
            position: pos,
            target_label: label.to_string(),
            width: 2,
        });
    }

    /// Emit PUSH2 <placeholder> JUMPI targeting a named label.
    /// Expects the condition to already be on the stack.
    pub fn emit_jumpi_to(&mut self, label: &str) {
        self.code.push(opcodes::PUSH2);
        let pos = self.code.len();
        self.code.extend_from_slice(&[0x00, 0x00]);
        self.code.push(opcodes::JUMPI);
        self.pending_jumps.push(PendingJump {
            position: pos,
            target_label: label.to_string(),
            width: 2,
        });
    }

    /// Patch all pending jump placeholders with resolved label offsets.
    pub fn resolve(&mut self) -> Result<(), CompileError> {
        for jump in &self.pending_jumps {
            let offset = self
                .labels
                .get(&jump.target_label)
                .ok_or_else(|| {
                    CompileError::semantic(
                        format!("Unresolved label '{}'", jump.target_label),
                        0,
                        0,
                        "",
                    )
                })?;

            let offset_bytes = (*offset as u16).to_be_bytes();
            self.code[jump.position] = offset_bytes[0];
            self.code[jump.position + 1] = offset_bytes[1];
        }
        Ok(())
    }

    /// Consume the emitter and return the final bytecode.
    pub fn into_bytes(self) -> Vec<u8> {
        self.code
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
