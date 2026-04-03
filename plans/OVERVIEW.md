# Toke

A domain-specific language that compiles declarative token definitions into EVM bytecode. Define an ERC-20 token in a few lines, get deployable bytecode — no Solidity required.

## Example

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

### Milestone 7: Code Generator
- [ ] `name()`, `symbol()`, `decimals()`
- [ ] Conditional `mint()` / `burn()` based on flags
- [ ] Supply cap enforcement in mint
- [ ] `approve()`, `allowance()`, `transferFrom()`
