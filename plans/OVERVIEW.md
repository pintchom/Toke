# Toke — A DSL for ERC-20 Token Contracts

## Overview

Toke is a domain-specific language that compiles declarative token definitions into EVM bytecode. The goal is to let users define ERC-20 token contracts in a few lines and get deployable bytecode — no Solidity knowledge required.

**File extension:** `.tc`
**CLI tool name:** `toke`
**Implementation language:** Rust

---

## 1. Language Specification

### 1.1 Syntax

A `.tc` file contains a single contract definition wrapped in a top-level `contract` block:

```
contract MyToken {
  symbol "MTK"
  decimals 18
  supply 1000000
  mintable
  burnable
}
```

### 1.2 Keywords and Fields

| Keyword    | Type     | Required | Default                  | Description                          |
|------------|----------|----------|--------------------------|--------------------------------------|
| `contract` | block    | yes      | —                        | Top-level container, followed by name and `{}` |
| `symbol`   | string   | no       | Derived from contract name (first 3 chars, uppercased) | Token ticker symbol |
| `decimals` | integer  | no       | `18`                     | Number of decimal places             |
| `supply`   | integer  | yes      | —                        | Total token supply                   |
| `mintable` | flag     | no       | `false`                  | Whether new tokens can be minted     |
| `burnable` | flag     | no       | `false`                  | Whether tokens can be burned         |
| `capped`   | integer  | no       | —                        | Maximum supply cap (only valid if mintable) |
| `owner`    | address  | no       | Deployer (`msg.sender`)  | Contract owner address               |

### 1.3 Data Types

| Type      | Format                | Examples                                      |
|-----------|-----------------------|-----------------------------------------------|
| string    | Double-quoted         | `"MyToken"`, `"MTK"`                          |
| integer   | Positive whole number  | `18`, `1000000`                               |
| address   | `0x` + 40 hex chars   | `0x1234567890abcdef1234567890abcdef12345678`   |
| flag      | Bare keyword          | `mintable`, `burnable`                        |

### 1.4 Grammar (EBNF)

```ebnf
program      ::= contract_decl

contract_decl ::= "contract" IDENTIFIER "{" field_list "}"

field_list   ::= (field)*

field        ::= symbol_field
               | decimals_field
               | supply_field
               | owner_field
               | capped_field
               | flag_field

symbol_field   ::= "symbol" STRING_LIT
decimals_field ::= "decimals" INTEGER_LIT
supply_field   ::= "supply" INTEGER_LIT
owner_field    ::= "owner" ADDRESS_LIT
capped_field   ::= "capped" INTEGER_LIT
flag_field     ::= "mintable" | "burnable"

IDENTIFIER   ::= [a-zA-Z_][a-zA-Z0-9_]*
STRING_LIT   ::= '"' [^"]* '"'
INTEGER_LIT  ::= [0-9]+
ADDRESS_LIT  ::= '0x' [0-9a-fA-F]{40}
```

### 1.5 Comments

```
# This is a comment
contract MyToken {
  supply 1000000  # inline comment
}
```

Single-line comments starting with `#`. No block comments.

---

## 2. Compiler Pipeline

```
Source (.tc file)
    │
    ▼
┌──────────┐
│  Lexer   │  → Vec<Token>
└──────────┘
    │
    ▼
┌──────────┐
│  Parser  │  → AST (ContractNode)
└──────────┘
    │
    ▼
┌──────────┐
│ Semantic │  → Validated AST
│ Analyzer │
└──────────┘
    │
    ▼
┌──────────┐
│  Code    │  → Vec<u8> (EVM bytecode)
│ Generator│
└──────────┘
    │
    ▼
  Output (.bin or hex string)
```

### 2.1 Lexer

**Purpose:** Convert raw source text into a stream of tokens.

**Token types:**

```rust
#[derive(Debug, Clone, PartialEq)]
enum TokenType {
    // Keywords
    Contract,
    Symbol,
    Decimals,
    Supply,
    Mintable,
    Burnable,
    Capped,
    Owner,

    // Literals
    StringLit,    // "MyToken"
    IntegerLit,   // 1000000
    AddressLit,   // 0xABC...

    // Punctuation
    OpenBrace,    // {
    CloseBrace,   // }

    // Meta
    Comment,      // # ...
    Newline,
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    col: usize,
}
```

**Responsibilities:**
- Walk through source character by character
- Recognize keywords, literals, and punctuation
- Skip whitespace (except newlines if significant)
- Skip comments but track position
- Attach line/col position to every token for error reporting
- Report errors for unrecognized characters

### 2.2 Parser

**Purpose:** Consume tokens and build an AST.

**AST definition:**

```rust
#[derive(Debug)]
struct ContractNode {
    name: String,
    name_span: Span,
    symbol: Option<StringField>,
    decimals: Option<IntField>,
    supply: Option<IntField>,
    mintable: Option<FlagField>,
    burnable: Option<FlagField>,
    capped: Option<IntField>,
    owner: Option<AddressField>,
}

#[derive(Debug)]
struct Span {
    line: usize,
    col: usize,
}

#[derive(Debug)]
struct StringField {
    value: String,
    span: Span,
}

#[derive(Debug)]
struct IntField {
    value: u64,
    span: Span,
}

#[derive(Debug)]
struct AddressField {
    value: [u8; 20],
    span: Span,
}

#[derive(Debug)]
struct FlagField {
    span: Span,
}
```

**Approach:** Recursive descent parser. For a language this simple, it's nearly flat — parse the `contract` keyword, the name, the opening brace, then loop through fields until the closing brace.

**Responsibilities:**
- Enforce structural rules (contract must have name, braces must match)
- Reject duplicate fields
- Store span info on every node for error reporting later

### 2.3 Semantic Analyzer

**Purpose:** Validate the AST for logical correctness beyond syntax.

**Checks to perform:**

| Check                                  | Error message                                                |
|----------------------------------------|--------------------------------------------------------------|
| `supply` must be present               | `Missing required field 'supply'`                            |
| `supply` must be > 0                   | `Supply must be a positive integer`                          |
| `decimals` must be 0–77                | `Decimals must be between 0 and 77`                          |
| `capped` without `mintable`            | `'capped' requires 'mintable' — a cap only applies if new tokens can be minted` |
| `capped` value < `supply`              | `Cap (X) is less than initial supply (Y)`                    |
| `symbol` length check                  | `Symbol 'X' is unusually long. Most tokens use 3-5 characters` (warning) |
| `decimals` unusual value               | `Decimals is set to X. Most tokens use 18. Are you sure?` (warning) |
| `owner` invalid address length         | `Address must be 40 hex characters (20 bytes)`               |

### 2.4 Code Generator

**Purpose:** Walk the validated AST and emit EVM bytecode.

#### 2.4.1 EVM Background

The EVM is a stack-based virtual machine. Key concepts:

- **Opcodes**: Single-byte instructions (e.g., `ADD = 0x01`, `PUSH1 = 0x60`)
- **Stack**: Values are pushed/popped, most operations consume the top N items
- **Memory**: Temporary byte-addressable space, used for ABI encoding return values
- **Storage**: Persistent key-value store (256-bit keys → 256-bit values), survives between calls
- **Calldata**: The input data sent with a transaction, first 4 bytes = function selector

#### 2.4.2 Opcode Table

```rust
#[allow(dead_code)]
mod opcodes {
    pub const STOP: u8         = 0x00;
    pub const ADD: u8          = 0x01;
    pub const MUL: u8          = 0x02;
    pub const SUB: u8          = 0x03;
    pub const DIV: u8          = 0x04;
    pub const LT: u8           = 0x10;
    pub const GT: u8           = 0x11;
    pub const EQ: u8           = 0x14;
    pub const ISZERO: u8       = 0x15;
    pub const SHA3: u8         = 0x20;
    pub const ADDRESS: u8      = 0x30;
    pub const CALLER: u8       = 0x33;
    pub const CALLVALUE: u8    = 0x34;
    pub const CALLDATALOAD: u8 = 0x35;
    pub const CALLDATASIZE: u8 = 0x36;
    pub const MSTORE: u8       = 0x52;
    pub const SLOAD: u8        = 0x54;
    pub const SSTORE: u8       = 0x55;
    pub const JUMP: u8         = 0x56;
    pub const JUMPI: u8        = 0x57;
    pub const JUMPDEST: u8     = 0x5b;
    pub const PUSH1: u8        = 0x60;
    pub const PUSH2: u8        = 0x61;
    pub const PUSH3: u8        = 0x62;
    pub const PUSH4: u8        = 0x63;
    pub const PUSH20: u8       = 0x73;
    pub const PUSH32: u8       = 0x7f;
    pub const DUP1: u8         = 0x80;
    pub const DUP2: u8         = 0x81;
    pub const SWAP1: u8        = 0x90;
    pub const LOG1: u8         = 0xa1;
    pub const RETURN: u8       = 0xf3;
    pub const REVERT: u8       = 0xfd;
    pub const SHR: u8          = 0x1c;
    pub const INVALID: u8      = 0xfe;
}
```

#### 2.4.3 Storage Layout

| Slot | Contents                                    |
|------|---------------------------------------------|
| 0    | `totalSupply` (uint256)                     |
| 1    | `name` (string — stored as bytes)           |
| 2    | `symbol` (string — stored as bytes)         |
| 3    | `decimals` (uint8)                          |
| 4    | `owner` (address, if applicable)            |
| 5    | `balanceOf` mapping base slot               |
| 6    | `allowance` mapping base slot               |

**Mapping storage:** For `balanceOf[address]`, the storage slot is `keccak256(abi.encode(address, 5))`. For `allowance[owner][spender]`, it's `keccak256(abi.encode(spender, keccak256(abi.encode(owner, 6))))`.

#### 2.4.4 Function Selectors (ABI)

Each public function is identified by the first 4 bytes of `keccak256(signature)`:

| Function                                          | Selector     |
|---------------------------------------------------|-------------|
| `name()`                                          | `0x06fdde03` |
| `symbol()`                                        | `0x95d89b41` |
| `decimals()`                                      | `0x313ce567` |
| `totalSupply()`                                   | `0x18160ddd` |
| `balanceOf(address)`                              | `0x70a08231` |
| `transfer(address,uint256)`                       | `0xa9059cbb` |
| `approve(address,uint256)`                        | `0x095ea7b3` |
| `allowance(address,address)`                      | `0xdd62ed3e` |
| `transferFrom(address,address,uint256)`           | `0x23b872dd` |
| `mint(address,uint256)` (if mintable)             | `0x40c10f19` |
| `burn(uint256)` (if burnable)                     | `0x42966c68` |

#### 2.4.5 Bytecode Structure

The compiler emits two chunks of bytecode:

**A. Constructor (deployment) bytecode:**
1. Store initial values in contract storage (supply, name, symbol, decimals, owner)
2. Credit entire supply to deployer's balance (`balanceOf[msg.sender] = supply`)
3. Copy the runtime bytecode into memory
4. Return the runtime bytecode (this becomes the deployed contract code)

**B. Runtime bytecode (deployed on-chain):**
1. **Function dispatcher:** Read first 4 bytes of calldata, compare against known selectors, jump to matching handler
2. **Function handlers:** One handler per supported function, each ends with RETURN or REVERT
3. **Fallback:** If no selector matches, REVERT

#### 2.4.6 Jump Offset Resolution (Two-Pass)

Because JUMP/JUMPI targets must be exact byte positions, the code generator uses two passes:

**Pass 1:** Emit all bytecode with placeholder values (e.g., `0x0000`) for jump targets. Record the byte offset of every `JUMPDEST`.

**Pass 2:** Go back and replace all placeholders with actual offsets now that positions are known.

```rust
struct Label {
    name: String,
    offset: Option<usize>,  // filled in during pass 1
}

struct PendingJump {
    position: usize,         // where the placeholder is in the bytecode
    target_label: String,    // which label to jump to
    width: usize,            // how many bytes the offset takes (PUSH1 = 1, PUSH2 = 2)
}
```

#### 2.4.7 Implementation Tiers

**Tier 1 — Minimum Viable (proves the pipeline works):**
- Constructor stores `totalSupply` in storage slot 0
- Constructor stores deployer balance in mapping slot
- Runtime dispatcher routes `totalSupply()` and `balanceOf(address)`
- `totalSupply()` → SLOAD slot 0, return
- `balanceOf(address)` → compute mapping slot via keccak256, SLOAD, return

**Tier 2 — Solid Project (adds state changes):**
- `transfer(address, uint256)` → check balance, subtract from sender, add to recipient, return true
- `name()`, `symbol()`, `decimals()` → return stored metadata
- Emit `Transfer` event via LOG

**Tier 3 — Impressive (conditional codegen):**
- `mintable` flag → conditionally emit `mint(address, uint256)` handler
- `burnable` flag → conditionally emit `burn(uint256)` handler
- `capped` → add supply cap check inside mint
- `approve`, `allowance`, `transferFrom` for full ERC-20 compliance

---

## 3. Error Handling

### 3.1 Error Structure

```rust
#[derive(Debug)]
struct CompileError {
    kind: ErrorKind,
    message: String,
    line: usize,
    col: usize,
    source_line: String,
    suggestion: Option<String>,
}

#[derive(Debug)]
enum ErrorKind {
    LexerError,
    ParseError,
    SemanticError,
    Warning,
}
```

### 3.2 Error Display Format

All errors follow a consistent format:

```
Error [line N, col M]: <message>

  N | <source line>
      <pointer>
  <suggestion>
```

Example:

```
Error [line 3, col 10]: Unexpected character '@'

  3 |   symbol @invalid
              ^
  Expected a string value like "MTK"
```

### 3.3 Error Catalog

#### Lexer Errors

| Condition                     | Message                                           |
|-------------------------------|---------------------------------------------------|
| Unrecognized character        | `Unexpected character 'X'`                        |
| Unterminated string           | `Unterminated string literal — missing closing '"'`|
| Invalid number (e.g., `12ab`) | `Invalid number literal '12ab'`                   |
| Invalid address format        | `Invalid address — expected '0x' followed by 40 hex characters` |

#### Parser Errors

| Condition                     | Message                                           |
|-------------------------------|---------------------------------------------------|
| Missing contract keyword      | `Expected 'contract' keyword`                     |
| Missing contract name         | `Missing contract name after 'contract'`          |
| Missing opening brace         | `Expected '{' after contract name`                |
| Missing closing brace         | `Expected '}' to close contract block`            |
| Unknown field keyword         | `Unknown field 'X'. Did you mean 'Y'?`            |
| Duplicate field               | `Duplicate 'X' declaration — already set on line N` |
| Wrong value type              | `'supply' expects a number, got string "abc"`     |

#### Semantic Errors

| Condition                     | Message                                           |
|-------------------------------|---------------------------------------------------|
| Missing supply                | `Missing required field 'supply'`                 |
| Zero supply                   | `Supply must be greater than 0`                   |
| Capped without mintable       | `'capped' requires 'mintable'`                    |
| Cap less than supply          | `Cap (X) is less than initial supply (Y)`         |

#### Warnings

| Condition                     | Message                                           |
|-------------------------------|---------------------------------------------------|
| Unusual decimals              | `Decimals is set to X. Most tokens use 18`        |
| Long symbol                   | `Symbol 'X' is unusually long. Most tokens use 3-5 characters` |
| Very large supply             | `Supply is very large. Make sure this is intentional` |

### 3.4 Fuzzy Matching for Suggestions

When the parser sees an unknown keyword, it should suggest the closest valid keyword using edit distance (Levenshtein distance):

```
Error [line 2]: Unknown field 'supplly'

  2 |   supplly 1000000
      ^^^^^^^
  Did you mean 'supply'?
```

---

## 4. CLI Tool

### 4.1 Commands

```bash
toke build <file.tc>               # Compile to bytecode, output to <file>.bin
toke build <file.tc> --hex         # Output bytecode as hex string to stdout
toke build <file.tc> -o out.bin    # Output to specific file
toke build <file.tc> --verbose     # Show compilation steps
toke lint <file.tc>                # Check for errors/warnings without compiling
toke init                          # Interactive wizard to generate a .tc file
toke --version                     # Print version
toke --help                        # Print help
```

### 4.2 `toke build`

Runs the full pipeline: lex → parse → analyze → generate → write output.

**Default output:** `<input_name>.bin` in the same directory.

**`--hex` flag:** Prints the bytecode as a `0x`-prefixed hex string to stdout instead of writing a binary file. Useful for copy-pasting into deployment tools.

**`--verbose` flag:** Prints intermediate info:

```
$ toke build mytoken.tc --verbose

  Lexing .............. 12 tokens
  Parsing ............. AST built
    name:     MyToken
    symbol:   MTK
    decimals: 18
    supply:   1000000
    mintable: true
    burnable: false
  Analyzing ........... 0 errors, 0 warnings
  Generating .......... 186 bytes
  Output: mytoken.bin
```

**Exit codes:**
- `0` — success
- `1` — compilation error
- `2` — file not found or I/O error

### 4.3 `toke lint`

Runs lex → parse → analyze only (no code generation). Reports all errors and warnings.

```
$ toke lint mytoken.tc

  ✓ No errors found
  1 warning:
    Warning [line 3]: Decimals is set to 30. Most tokens use 18.
```

### 4.4 `toke init` — Interactive Wizard

Walks the user through contract creation step by step:

```
$ toke init

  🔥 Welcome to Toke

  ? Contract name: MyToken
  ? Token symbol (default: MYT): MTK
  ? Decimals (default: 18): 18
  ? Total supply: 1000000
  ? Mintable? (y/n): y
  ? Max supply cap? (leave blank for no cap): 5000000
  ? Burnable? (y/n): n

  ✅ Generated mytoken.tc

  contract MyToken {
    symbol "MTK"
    decimals 18
    supply 1000000
    mintable
    capped 5000000
  }

  Run 'toke build mytoken.tc' to compile.
```

**Validation during wizard:**
- Contract name must be a valid identifier
- Symbol must be non-empty string
- Decimals must be 0–77 (warn if not 18)
- Supply must be positive integer
- Cap must be >= supply (if provided)

**Rust crates for CLI:**
- `clap` — argument parsing and subcommands
- `dialoguer` — interactive prompts (Input, Confirm, Select)
- `console` — colored terminal output
- `indicatif` — progress indicators (optional, for verbose mode)

---

## 5. Project Structure

```
toke/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # CLI entry point, argument parsing
│   ├── lib.rs                # Public compile() function
│   ├── lexer.rs              # Tokenizer
│   ├── tokens.rs             # Token and TokenType definitions
│   ├── parser.rs             # Recursive descent parser
│   ├── ast.rs                # AST node definitions
│   ├── analyzer.rs           # Semantic analysis and validation
│   ├── codegen/
│   │   ├── mod.rs            # Code generator entry point
│   │   ├── opcodes.rs        # EVM opcode constants
│   │   ├── dispatcher.rs     # Function selector routing
│   │   ├── constructor.rs    # Deployment bytecode
│   │   ├── functions.rs      # Individual function handlers
│   │   └── utils.rs          # ABI encoding, keccak helpers
│   ├── errors.rs             # Error types and formatting
│   └── init.rs               # Interactive wizard
├── tests/
│   ├── lexer_tests.rs
│   ├── parser_tests.rs
│   ├── analyzer_tests.rs
│   ├── codegen_tests.rs
│   └── integration_tests.rs
└── examples/
    ├── basic.tc
    ├── mintable.tc
    └── full.tc
```

---

## 6. Testing Strategy

### 6.1 Unit Tests

**Lexer tests:**
- Valid token sequences
- Each literal type (string, integer, address)
- Comments are skipped
- Error on invalid characters
- Position tracking is correct

**Parser tests:**
- Minimal valid contract
- All optional fields
- Missing required fields → correct error
- Duplicate fields → correct error
- Wrong value types → correct error

**Analyzer tests:**
- Missing supply → error
- Zero supply → error
- Capped without mintable → error
- Unusual decimals → warning
- All valid combinations pass

**Codegen tests:**
- Generated bytecode contains correct function selectors
- Jump offsets are valid (every JUMPI target has a JUMPDEST)
- Supply value is correctly encoded in constructor

### 6.2 Integration Tests

- Compile a `.tc` file end-to-end and verify output bytecode
- Run generated bytecode on a local EVM (using a Rust EVM crate like `revm`) and call each function to verify correct behavior
- Test that `totalSupply()` returns the right value
- Test that `balanceOf(deployer)` equals supply
- Test that `transfer()` moves tokens correctly

### 6.3 Error Tests

- Verify every error in the error catalog is triggered by the right input
- Verify error messages include correct line/col
- Verify suggestions appear for typos

---

## 7. Dependencies (Cargo.toml)

```toml
[package]
name = "toke"
version = "0.1.0"
edition = "2021"
description = "A DSL that compiles to ERC-20 EVM bytecode"

[dependencies]
clap = { version = "4", features = ["derive"] }     # CLI argument parsing
dialoguer = "0.11"                                    # Interactive prompts
console = "0.15"                                      # Colored terminal output
tiny-keccak = { version = "2", features = ["keccak"] } # keccak256 for storage slots and selectors
hex = "0.4"                                           # Hex encoding/decoding
thiserror = "2"                                       # Error derive macros
strsim = "0.11"                                       # String similarity for "did you mean?" suggestions

[dev-dependencies]
revm = "3"                                            # EVM interpreter for integration tests
```

---

## 8. Milestones

### Milestone 1: Lexer + Tokens
- [ ] Define all token types
- [ ] Implement lexer with position tracking
- [ ] Handle strings, integers, addresses, keywords, braces, comments
- [ ] Lexer error reporting with line/col
- [ ] Unit tests for lexer

### Milestone 2: Parser + AST
- [ ] Define AST structures
- [ ] Implement recursive descent parser
- [ ] Duplicate field detection
- [ ] "Did you mean?" suggestions for unknown keywords
- [ ] Parser error reporting
- [ ] Unit tests for parser

### Milestone 3: Semantic Analyzer
- [ ] Required field validation
- [ ] Value range checks
- [ ] Cross-field logic (capped requires mintable)
- [ ] Warnings for unusual values
- [ ] Unit tests for analyzer

### Milestone 4: Code Generator (Tier 1)
- [ ] Opcode table
- [ ] Constructor: store supply, set deployer balance
- [ ] Function dispatcher
- [ ] `totalSupply()` handler
- [ ] `balanceOf(address)` handler
- [ ] Two-pass jump resolution
- [ ] Integration test: deploy and call on local EVM

### Milestone 5: CLI
- [ ] `toke build` with --hex, -o, --verbose flags
- [ ] `toke lint`
- [ ] Error formatting with color
- [ ] Exit codes

### Milestone 6: Interactive Wizard
- [ ] `toke init` with prompts and validation
- [ ] File generation
- [ ] Inline validation during prompts

### Milestone 7: Code Generator (Tier 2) — stretch
- [ ] `transfer()` with balance checks
- [ ] `name()`, `symbol()`, `decimals()`
- [ ] Transfer event emission

### Milestone 8: Code Generator (Tier 3) — stretch
- [ ] Conditional `mint()` / `burn()` based on flags
- [ ] Supply cap enforcement in mint
- [ ] `approve()`, `allowance()`, `transferFrom()`

---

## 9. Example Session

```bash
$ toke init

  🔥 Welcome to Toke

  ? Contract name: CoolCoin
  ? Token symbol (default: COO): COOL
  ? Decimals (default: 18): 18
  ? Total supply: 1000000
  ? Mintable? (y/n): n
  ? Burnable? (y/n): n

  ✅ Generated coolcoin.tc

$ cat coolcoin.tc

  contract CoolCoin {
    symbol "COOL"
    decimals 18
    supply 1000000
  }

$ toke lint coolcoin.tc

  ✓ No errors found, no warnings

$ toke build coolcoin.tc --hex --verbose

  Lexing .............. 10 tokens
  Parsing ............. AST built
    name:     CoolCoin
    symbol:   COOL
    decimals: 18
    supply:   1000000
    mintable: false
    burnable: false
  Analyzing ........... 0 errors, 0 warnings
  Generating .......... 142 bytes

  0x6080604052...  (full bytecode hex)
```

---

## 10. GitHub Language Recognition

### 10.1 Basic Setup

Add a `.gitattributes` file to the repo root so GitHub recognizes `.tc` files and applies syntax highlighting:

```gitattributes
*.tc linguist-language=Ruby
```

Ruby highlighting works well because it has similar keyword-value structure. HCL (Terraform) is another good option:

```gitattributes
*.tc linguist-language=HCL
```

### 10.2 Full Language Registration (stretch goal)

To get Toke officially recognized on GitHub as its own language, you'd submit a PR to [github-linguist](https://github.com/github-linguist/linguist) with:

- A TextMate grammar (`.tmLanguage.json`) defining syntax rules for highlighting
- A sample `.tc` file
- An entry in `languages.yml` with the file extension, color, and grammar reference

The TextMate grammar would define scopes for:

| Scope                        | Matches                                      |
|------------------------------|----------------------------------------------|
| `keyword.control.toke`       | `contract`                                   |
| `keyword.other.toke`         | `symbol`, `decimals`, `supply`, `owner`, `capped` |
| `keyword.modifier.toke`      | `mintable`, `burnable`                       |
| `string.quoted.double.toke`  | `"MyToken"`, `"MTK"`                         |
| `constant.numeric.toke`      | `18`, `1000000`                              |
| `constant.other.address.toke`| `0x1234...`                                  |
| `comment.line.hash.toke`     | `# comments`                                 |
| `punctuation.section.block`  | `{`, `}`                                     |

This is overkill for a class project but would be a nice portfolio piece. For now, the `.gitattributes` approach is sufficient.