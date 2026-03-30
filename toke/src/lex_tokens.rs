/**
 * Token Declaration info
 * name
 * supply
 * symbol
 * mintable
 * burnable
 * capped
 * owner
 * decimals
 */

pub enum LexTokenType {
    Contract, // token declaration: contract MyToken {...}
    Symbol,   // Symbol declaration: symbol "..."
    Decimals, // Decimals declaration: decimals ...(int parsed as string)
    Supply,   // Supply declaration: supply ...(int parsed as string)
    Mintable, // Mintable declaration: mintable (opt true/false, exclusion = false, inclusion = true)
    Burnable, // Burnable declaration: burnable (opt true/false, exclusion = false, inclusion = true)
    Capped,   // Capped declaration: capped ...(int parsed as string)
    Owner,    // Owner declaration: owner ...(address parsed as string)

    Identifier(String), // identifier assignments: MyToken
    StringLit(String),  // string assignments: "asd"
    IntegerLit(String), //
    AddressLit { bytes: [u8; 20], raw: String },

    OpenBrace,
    CloseBrace,

    EOF,
}

pub struct LexToken {
    pub token_type: LexTokenType, // one of the above lexer items
    pub line: usize,              // line destination
    pub col: usize,               // indendtation destination
}
