#![allow(dead_code)]

// Arithmetic
pub const STOP: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;

// Comparison
pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;

// Bitwise
pub const SHR: u8 = 0x1c;

// Hashing
pub const SHA3: u8 = 0x20;

// Environment
pub const ADDRESS: u8 = 0x30;
pub const CALLER: u8 = 0x33;
pub const CALLVALUE: u8 = 0x34;
pub const CALLDATALOAD: u8 = 0x35;
pub const CALLDATASIZE: u8 = 0x36;

// Memory / Storage
pub const CODECOPY: u8 = 0x39;
pub const POP: u8 = 0x50;
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const SLOAD: u8 = 0x54;
pub const SSTORE: u8 = 0x55;

// Control flow
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const JUMPDEST: u8 = 0x5b;

// Push instructions (PUSH1 through PUSH32 = 0x60 through 0x7f)
pub const PUSH1: u8 = 0x60;
pub const PUSH2: u8 = 0x61;
pub const PUSH3: u8 = 0x62;
pub const PUSH4: u8 = 0x63;
pub const PUSH20: u8 = 0x73;
pub const PUSH32: u8 = 0x7f;

// Duplication
pub const DUP1: u8 = 0x80;
pub const DUP2: u8 = 0x81;

// Exchange
pub const SWAP1: u8 = 0x90;

// Logging
pub const LOG1: u8 = 0xa1;

// System
pub const RETURN: u8 = 0xf3;
pub const REVERT: u8 = 0xfd;
pub const INVALID: u8 = 0xfe;
