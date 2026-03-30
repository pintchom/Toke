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
        //  * letter or underscore -> call self.read_code_word()
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
        while let Some(c) = self.current_char() {
            match c {
                c if c.is_ascii_whitespace() => self.skip_whitespace(), // will be back on the next valid non empty char
                '#' => self.skip_comment(), // will be back on the next line
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
                '"' => {
                    let lex_token = self.read_string()?;
                    lex_tokens.push(lex_token);
                }
                '0' if self.source.get(self.pos + 1) == Some(&'x') => {
                    let lex_token = self.read_address()?;
                    lex_tokens.push(lex_token)
                }
                c if c.is_ascii_digit() => {
                    let lex_token = self.read_number()?;
                    lex_tokens.push(lex_token);
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let lex_token = self.read_code_word()?;
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
        while let Some(c) = self.current_char() {
            if c.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
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
        let start_line = self.line;
        let start_col = self.col;
        let mut num_str = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance();
            } else if c.is_ascii_whitespace() || c == '}' {
                break;
            } else {
                return Err(format!(
                    "Invalid number literal '{}{}' at line {}, column {}",
                    num_str, c, start_line, start_col
                ));
            }
        }

        Ok(self.make_token(LexTokenType::IntegerLit(num_str), start_line, start_col))
    }
    fn read_code_word(&mut self) -> Result<LexToken, String> {
        let start_line = self.line;
        let start_col = self.col;
        let mut word_str = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_alphanumeric() || c == '_' {
                word_str.push(c);
                self.advance();
            } else if c.is_ascii_whitespace() || c == '{' || c == '}' {
                break;
            } else {
                return Err(format!(
                    "Invalid character '{}' in identifier '{}' at line {}, column {}",
                    c, word_str, start_line, start_col
                ));
            }
        }

        let token_type = match word_str.as_str() {
            "contract" => LexTokenType::Contract,
            "symbol" => LexTokenType::Symbol,
            "decimals" => LexTokenType::Decimals,
            "supply" => LexTokenType::Supply,
            "mintable" => LexTokenType::Mintable,
            "burnable" => LexTokenType::Burnable,
            "capped" => LexTokenType::Capped,
            "owner" => LexTokenType::Owner,
            _ => LexTokenType::Identifier(word_str),
        };

        Ok(self.make_token(token_type, start_line, start_col))
    }

    fn read_string(&mut self) -> Result<LexToken, String> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance(); // skip opening "
        let mut string_str = String::new();

        loop {
            match self.current_char() {
                Some('"') => {
                    self.advance(); // skip closing "
                    return Ok(self.make_token(
                        LexTokenType::StringLit(string_str),
                        start_line,
                        start_col,
                    ));
                }
                Some('\n') => {
                    return Err(format!(
                        "Unterminated string literal at line {}, column {}",
                        start_line, start_col
                    ));
                }
                Some(c) => {
                    string_str.push(c);
                    self.advance();
                }
                None => {
                    return Err(format!(
                        "Unterminated string literal at line {}, column {}",
                        start_line, start_col
                    ));
                }
            }
        }
    }
    fn read_address(&mut self) -> Result<LexToken, String> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance(); // skip '0'
        self.advance(); // skip 'x'
        let mut hex_str = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_hexdigit() {
                hex_str.push(c);
                self.advance();
            } else if c.is_ascii_whitespace() || c == '}' {
                break;
            } else {
                return Err(format!(
                    "Invalid character '{}' in address at line {}, column {}",
                    c, start_line, start_col
                ));
            }
        }

        if hex_str.len() != 40 {
            return Err(format!(
                "Address must be 40 hex characters (got {}), at line {}, column {}",
                hex_str.len(),
                start_line,
                start_col
            ));
        }

        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16).map_err(|_| {
                format!(
                    "Invalid hex in address at line {}, column {}",
                    start_line, start_col
                )
            })?;
        }

        let raw = format!("0x{}", hex_str);
        Ok(self.make_token(
            LexTokenType::AddressLit { bytes, raw },
            start_line,
            start_col,
        ))
    }
}
