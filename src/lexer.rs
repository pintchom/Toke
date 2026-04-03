use crate::errors::{get_source_line, CompileError};
use crate::lex_tokens::{LexToken, LexTokenType};

pub struct Lexer {
    source: Vec<char>,
    raw_source: String,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            raw_source: source.to_string(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<LexToken>, CompileError> {
        let mut lex_tokens: Vec<LexToken> = Vec::new();
        while let Some(c) = self.current_char() {
            match c {
                c if c.is_ascii_whitespace() => self.skip_whitespace(), // will be back on the next valid non empty char
                '#' => self.skip_comment(), // will be back on the next line
                '{' => {
                    lex_tokens.push(self.make_token(LexTokenType::OpenBrace, self.line, self.col));
                    self.advance();
                }
                '}' => {
                    lex_tokens.push(self.make_token(LexTokenType::CloseBrace, self.line, self.col));
                    self.advance();
                }
                '"' => lex_tokens.push(self.read_string()?),
                '0' if self.source.get(self.pos + 1) == Some(&'x') => {
                    lex_tokens.push(self.read_address()?)
                }
                c if c.is_ascii_digit() => lex_tokens.push(self.read_number()?),
                c if c.is_ascii_alphabetic() || c == '_' => {
                    lex_tokens.push(self.read_code_word()?)
                }
                _ => {
                    return Err(CompileError::lexer(
                        format!("Unexpected character '{}'", c),
                        self.line,
                        self.col,
                        self.source_line(self.line),
                    ));
                }
            }
        }
        lex_tokens.push(self.make_token(LexTokenType::EOF, self.line, self.col));
        Ok(lex_tokens)
    }

    fn source_line(&self, line: usize) -> String {
        get_source_line(&self.raw_source, line)
    }

    fn make_token(&self, token_type: LexTokenType, line: usize, col: usize) -> LexToken {
        LexToken {
            token_type,
            line,
            col,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.source.get(self.pos).copied()
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

    fn read_number(&mut self) -> Result<LexToken, CompileError> {
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
                return Err(CompileError::lexer(
                    format!("Invalid number literal '{}{}'", num_str, c),
                    start_line,
                    start_col,
                    self.source_line(start_line),
                ));
            }
        }

        Ok(self.make_token(LexTokenType::IntegerLit(num_str), start_line, start_col))
    }

    fn read_code_word(&mut self) -> Result<LexToken, CompileError> {
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
                return Err(CompileError::lexer(
                    format!("Invalid character '{}' in identifier '{}'", c, word_str),
                    start_line,
                    start_col,
                    self.source_line(start_line),
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

    fn read_string(&mut self) -> Result<LexToken, CompileError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance();
        let mut string_str = String::new();

        loop {
            match self.current_char() {
                Some('"') => {
                    self.advance();
                    return Ok(self.make_token(
                        LexTokenType::StringLit(string_str),
                        start_line,
                        start_col,
                    ));
                }
                Some('\n') | None => {
                    return Err(CompileError::lexer(
                        "Unterminated string literal — missing closing '\"'",
                        start_line,
                        start_col,
                        self.source_line(start_line),
                    ));
                }
                Some(c) => {
                    string_str.push(c);
                    self.advance();
                }
            }
        }
    }

    fn read_address(&mut self) -> Result<LexToken, CompileError> {
        let start_line = self.line;
        let start_col = self.col;
        self.advance();
        self.advance();
        let mut hex_str = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_hexdigit() {
                hex_str.push(c);
                self.advance();
            } else if c.is_ascii_whitespace() || c == '}' {
                break;
            } else {
                return Err(CompileError::lexer(
                    format!("Invalid character '{}' in address", c),
                    start_line,
                    start_col,
                    self.source_line(start_line),
                ));
            }
        }

        if hex_str.len() != 40 {
            return Err(CompileError::lexer(
                format!(
                    "Address must be 40 hex characters (got {})",
                    hex_str.len()
                ),
                start_line,
                start_col,
                self.source_line(start_line),
            ));
        }

        let mut bytes = [0u8; 20];
        for i in 0..20 {
            bytes[i] = u8::from_str_radix(&hex_str[i * 2..i * 2 + 2], 16).map_err(|_| {
                CompileError::lexer(
                    "Invalid hex in address",
                    start_line,
                    start_col,
                    self.source_line(start_line),
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
