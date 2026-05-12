# Toke

A DSL that compiles declarative ERC-20 token definitions into deployable EVM bytecode. Write a few lines, get a contract — no Solidity required.

```
contract CoolCoin {
    symbol "COOL"
    decimals 18
    supply 5000000
    mintable
    capped 10000000
}
```

```bash
toke build coolcoin.tc --hex
# 0x6a0422ca8b0a00a4250000006000553360005260056020526040600020...
```

---

## Installation

Requires Rust 1.75 or later.

```bash
git clone https://github.com/pintchom/Toke
cd Toke
cargo install --path .
```

Verify it works:

```bash
toke --version
```

---

## Quick Start

**1. Write a `.tc` file:**

```
contract MyToken {
    symbol "MTK"
    decimals 18
    supply 1000000
}
```

**2. Lint it:**

```bash
toke lint mytoken.tc
# ✓ No errors found, no warnings
```

**3. Compile to bytecode:**

```bash
toke build mytoken.tc --hex
# 0x...
```

**4. Deploy** the hex output using any EVM-compatible tool (Foundry's `cast`, Hardhat, Remix, etc.).

---

## Language Reference

A `.tc` file contains a single `contract` block. The contract name becomes the token's `name()` return value.

```
contract <Name> {
    <fields>
}
```

### Fields

| Field      | Type    | Required | Default                              | Description                                      |
|------------|---------|----------|--------------------------------------|--------------------------------------------------|
| `supply`   | integer | **yes**  | —                                    | Initial token supply (whole tokens, scaled by decimals) |
| `symbol`   | string  | no       | First 3 chars of name, uppercased    | Ticker symbol, e.g. `"ETH"`                      |
| `decimals` | integer | no       | `18`                                 | Decimal places. Must be 0–77                     |
| `mintable` | flag    | no       | off                                  | Allows new tokens to be minted after deployment  |
| `burnable` | flag    | no       | off                                  | Allows token holders to burn their tokens        |
| `capped`   | integer | no       | —                                    | Maximum total supply. Requires `mintable`        |
| `owner`    | address | no       | Deployer (`msg.sender`)              | Contract owner address                           |

### Types

| Type    | Format                  | Example                                        |
|---------|-------------------------|------------------------------------------------|
| integer | Positive whole number   | `1000000`                                      |
| string  | Double-quoted           | `"MTK"`                                        |
| address | `0x` + 40 hex chars     | `0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef`   |
| flag    | Bare keyword, no value  | `mintable`                                     |

### Comments

```
# This is a comment
contract MyToken {
    supply 1000000  # inline comment
}
```

### Full Example

```
contract GoldToken {
    symbol "GLD"
    decimals 18
    supply 1000000
    mintable
    burnable
    capped 10000000
    owner 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
}
```

---

## CLI Reference

### `toke build <file.tc>`

Compiles a `.tc` file to EVM bytecode.

```bash
toke build mytoken.tc                    # writes mytoken.bin
toke build mytoken.tc --hex              # prints 0x... hex to stdout
toke build mytoken.tc -o output.bin      # custom output path
toke build mytoken.tc --verbose          # show compilation steps
```

**Flags:**

| Flag              | Description                                         |
|-------------------|-----------------------------------------------------|
| `--hex`           | Print bytecode as a `0x`-prefixed hex string to stdout instead of writing a `.bin` file |
| `-o`, `--output`  | Write output to a specific path                     |
| `--verbose`       | Print token count, parsed fields, and byte count    |

**Exit codes:** `0` success, `1` compile error, `2` I/O error.

---

### `toke lint <file.tc>`

Runs the full lex → parse → analyze pipeline without generating bytecode. Reports all errors and warnings.

```bash
toke lint mytoken.tc
```

Useful in CI or as a pre-commit check. Exit codes are the same as `build`.

---

### `toke init`

Interactive wizard to scaffold a new `.tc` file. *(Coming soon.)*

---

## Error Reference

Toke produces precise errors with line/column positions and source context:

```
Error [line 4, col 5]: 'capped' requires 'mintable' — a cap only applies if new tokens can be minted

  4 |     capped 5000000
      ^
```

### Errors

| Cause                          | Message                                                      |
|--------------------------------|--------------------------------------------------------------|
| Missing `supply`               | `Missing required field 'supply'`                            |
| `supply 0`                     | `Supply must be greater than 0`                              |
| `decimals` outside 0–77        | `Decimals must be between 0 and 77, got N`                   |
| `capped` without `mintable`    | `'capped' requires 'mintable'`                               |
| `capped` less than `supply`    | `Cap (X) is less than initial supply (Y)`                    |
| Duplicate field                | `Duplicate 'supply' declaration — already set on line N`     |
| Unknown field                  | `Unknown field 'supplly'. Did you mean 'supply'?`            |
| Unterminated string            | `Unterminated string literal — missing closing '"'`          |
| Invalid address length         | `Address must be 40 hex characters (got N)`                  |

### Warnings

Warnings don't stop compilation but are printed to stderr:

| Cause                   | Message                                          |
|-------------------------|--------------------------------------------------|
| `decimals` is not `18`  | `Decimals is set to N. Most tokens use 18`       |
| Symbol longer than 5 chars | `Symbol 'X' is unusually long. Most tokens use 3-5 characters` |

---

## How It Works

Toke compiles through four stages:

```
.tc source
    │
    ▼
  Lexer        tokenizes keywords, literals, punctuation
    │
    ▼
  Parser       builds an AST (ContractNode)
    │
    ▼
  Analyzer     validates fields and cross-field logic
    │
    ▼
  Codegen      emits constructor + runtime EVM bytecode
    │
    ▼
  .bin / hex
```

The output bytecode is split into two sections. The constructor runs once at deployment: it stores `totalSupply` in storage, credits the deployer's balance, emits a `Transfer` event, then returns the runtime code. The runtime contains a function dispatcher that routes calls by their 4-byte ABI selector, followed by handlers for all nine standard ERC-20 functions.

### Storage Layout

| Slot | Value                     |
|------|---------------------------|
| 0    | `totalSupply`             |
| 1–4  | reserved (name, symbol, decimals, owner) |
| 5    | `balanceOf` mapping base  |
| 6    | `allowance` mapping base  |

Mapping slots use the standard Solidity convention: `balanceOf[addr]` lives at `keccak256(abi.encode(addr, 5))`.

### Supported ERC-20 Functions

The generated contract implements the full ERC-20 interface:

| Function                                | Selector     |
|-----------------------------------------|--------------|
| `name()`                                | `06fdde03`   |
| `symbol()`                              | `95d89b41`   |
| `decimals()`                            | `313ce567`   |
| `totalSupply()`                         | `18160ddd`   |
| `balanceOf(address)`                    | `70a08231`   |
| `transfer(address,uint256)`             | `a9059cbb`   |
| `approve(address,uint256)`              | `095ea7b3`   |
| `allowance(address,address)`            | `dd62ed3e`   |
| `transferFrom(address,address,uint256)` | `23b872dd`   |

---

## Examples

Minimal token:
```
contract SimpleToken {
    supply 1000000
}
```

Stablecoin-style (6 decimals):
```
contract USDStable {
    symbol "USDS"
    decimals 6
    supply 1000000000
}
```

Mintable with a hard cap:
```
contract CappedToken {
    symbol "CAP"
    decimals 18
    supply 1000000
    mintable
    capped 100000000
}
```

Full configuration:
```
contract FullToken {
    symbol "FUL"
    decimals 18
    supply 1000000
    mintable
    burnable
    capped 50000000
    owner 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef
}
```

---

## Contributing

```bash
cargo test          # run all tests
cargo test --test cli_tests   # CLI integration tests only
cargo clippy        # lints
```

The project is structured as a library (`src/lib.rs`) with a thin binary entry point (`src/main.rs`), so all compilation logic is importable and testable without spawning a subprocess.

---

## License

MIT
