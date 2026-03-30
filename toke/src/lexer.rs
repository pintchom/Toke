use crate::lex_tokens::{LexToken, LexTokenType};
pub struct Lexer {
    source: Vec<char>, // source code as characters
    pos: usize,        // current position in the source code
    line: usize,       // current line number
    col: usize,        // current column number
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let source: Vec<char> = source.chars().collect();
        let pos: usize = 0;
        let line: usize = 1;
        let col: usize = 1;
        return Self {
            source,
            pos,
            line,
            col,
        };
    }

    pub fn tokenize(&mut self) -> Result<Vec<LexToken>, String> {
        // need customer error types here
        //  * Loop until you consume all characters
        //  * - Skip whitespaces (spaces, tabs, newlines)
        //  * - Look at the current character and edcide what to do with it:
        //  *  '{' push openBrace lex token
        //  *  '}' push closeBrace lexToken
        //  *  '"' call self.read_string()
        //  * '0' and next char is 'x' call self.read_address()
        //  * digit -> call self.read_number()
        //  * letter or underscore -> call self.read_word()
        //  * '#' call self.skip_comment
        //  *  anything else, return error with line/col
        //  *
        //  * Example:
        //  * # COMMENT
        //  * contract MyToken {
        //  *   supply 1000000
        //  * }
        //  *
        //  * This is main entry point in determine valid syntax

        let mut lex_tokens: Vec<LexToken> = Vec::new();
        while self.current_char().is_some() {
            let c = self.current_char().unwrap();
            match c {
                ' ' | '\t' | '\r' | '\n' => self.skip_whitespace(), // will be back on the next valid non empty char
                '#' => self.skip_comment(),                         // will be back on the next line
                '{' => {
                    let lex_token: LexToken =
                        self.make_token(LexTokenType::OpenBrace, self.line, self.col);
                    lex_tokens.push(lex_token);
                    self.advance();
                }
                '}' => {
                    let lex_token: LexToken =
                        self.make_token(LexTokenType::CloseBrace, self.line, self.col);
                    lex_tokens.push(lex_token);
                    self.advance();
                }
                '0' => {}
                c if c.is_ascii_digit() => {
                    let lex_token = self.read_number()?;
                    lex_tokens.push(lex_token);
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let lex_token = self.read_word();
                    lex_tokens.push(lex_token);
                }
                _ => {
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        self.current_char().unwrap(),
                        self.line,
                        self.col
                    ));
                }
            }
        }
        lex_tokens.push(self.make_token(LexTokenType::EOF, self.line, self.col));
        return Ok(lex_tokens);
    }

    fn make_token(&self, token_type: LexTokenType, line: usize, col: usize) -> LexToken {
        return LexToken {
            token_type,
            line,
            col,
        };
    }

    fn current_char(&self) -> Option<char> {
        if self.pos < self.source.len() {
            return Some(self.source[self.pos]);
        }
        return None;
    }

    fn advance(&mut self) {
        if self.current_char() == Some('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.pos += 1;
    }

    fn skip_whitespace(&mut self) {
        while matches!(
            self.current_char(),
            Some(' ') | Some('\t') | Some('\r') | Some('\n'),
        ) {
            self.advance();
        }
    }
    fn skip_comment(&mut self) {
        while self.current_char().is_some() && self.current_char() != Some('\n') {
            self.advance();
        }
        if self.current_char().is_some() {
            self.advance();
        }
    }
    fn read_number(&mut self) -> Result<LexToken, String> {
        // 1. Record starting line/col
        // 2. Collect characters while they are digits
        // 3. If you hit a letter (like "123abc"), return an error
        // 4. Return a LexToken with IntegerLit and the collected string
        todo!()
    }
    fn read_word(&mut self) -> LexToken {
        // 1. Record starting line/col
        // 2. Collect characters while they are letters, digits, or underscores
        // 3. Check the collected word against your keyword list:
        //    "contract" → Contract
        //    "symbol"   → Symbol
        //    "decimals" → Decimals
        //    "supply"   → Supply
        //    "mintable" → Mintable
        //    "burnable" → Burnable
        //    "capped"   → Capped
        //    "owner"    → Owner
        // 4. If it's not a keyword, it's an Identifier
        // 5. Return the appropriate LexToken
        todo!()
    }
    fn read_address(&mut self) -> Result<LexToken, String> {
        // 1. Record starting line/col
        // 2. Advance past '0' and 'x'
        // 3. Collect exactly 40 hex characters (0-9, a-f, A-F)
        // 4. If you get fewer than 40, return an error
        // 5. If you hit a non-hex character, return an error
        // 6. Convert the 40 hex chars to [u8; 20] bytes
        // 7. Return a LexToken with AddressLit containing both bytes and raw string
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lex_tokens::LexTokenType;

    #[test]
    fn test_simple_contract() {
        let mut lexer = Lexer::new("contract MyToken { }");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0].token_type, LexTokenType::Contract));
        assert!(matches!(tokens[1].token_type, LexTokenType::Identifier(ref s) if s == "MyToken"));
        assert!(matches!(tokens[2].token_type, LexTokenType::OpenBrace));
        assert!(matches!(tokens[3].token_type, LexTokenType::CloseBrace));
        assert!(matches!(tokens[4].token_type, LexTokenType::EOF));
    }
}
