# Toke

A domain-specific language that compiles declarative token definitions into EVM bytecode. Define an ERC-20 token in a few lines, get deployable bytecode — no Solidity required.

## Example

```
contract MyToken {
    symbol "MTK"
    decimals 18
    supply 1000000
    mintable
    burnable
}
```

## Pipeline

```
Source (.tc) → Lexer → Parser → Semantic Analyzer → Code Generator → Bytecode
```

## Status

- [x] Lexer — tokenizes `.tc` source files
- [x] Parser — builds AST from token stream
- [x] Semantic Analyzer — validates contract logic
- [ ] Code Generator — emits EVM bytecode
- [ ] CLI — `toke build`, `toke lint`, `toke init`

## Run

```bash
cargo test    # run all tests
cargo run     # run the compiler
```
