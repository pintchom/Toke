use std::collections::HashMap;

use crate::errors::CompileError;

use super::opcodes;

#[allow(dead_code)] // temporary for width prop
struct PendingJump {
    position: usize,
    target_label: String,
    width: usize, // unused for now since wehandle static 2 byte offsets
}

/// 1: emit bytecode with placeholder zeros for jump targets, recording
///          each label's byte offset and each pending jump's location.
/// 2: `resolve()` patches every placeholder with the real offset.
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

    pub fn emit(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    pub fn emit_label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.code.len());
        self.code.push(opcodes::JUMPDEST);
    }

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

    pub fn resolve(&mut self) -> Result<(), CompileError> {
        for jump in &self.pending_jumps {
            let offset = self.labels.get(&jump.target_label).ok_or_else(|| {
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

    pub fn into_bytes(self) -> Vec<u8> {
        self.code
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }
}
