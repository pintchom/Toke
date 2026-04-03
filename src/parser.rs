use crate::ast::{AddressField, ContractNode, FlagField, IntField, Position, StringField};
use crate::errors::{get_source_line, suggest_closest, CompileError};
use crate::lex_tokens::{LexToken, LexTokenType};

pub struct Parser {
    tokens: Vec<LexToken>,
    pos: usize,
    source: String,
}

impl Parser {
    pub fn new(tokens: Vec<LexToken>, source: &str) -> Self {
        Self {
            tokens,
            pos: 0,
            source: source.to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<ContractNode, CompileError> {
        self.expect("contract")?;
        let (name, name_position) = self.expect_identifier()?;
        self.expect("{")?;

        let mut contract = ContractNode {
            name,
            name_position,
            symbol: None,
            decimals: None,
            supply: None,
            mintable: None,
            burnable: None,
            capped: None,
            owner: None,
        };

        self.parse_fields(&mut contract)?;
        self.expect("}")?;
        self.expect("EOF")?;

        Ok(contract)
    }

    fn parse_fields(&mut self, contract: &mut ContractNode) -> Result<(), CompileError> {
        while !matches!(self.current().token_type, LexTokenType::CloseBrace) {
            match &self.current().token_type {
                LexTokenType::Supply => {
                    if let Some(ref existing) = contract.supply {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'supply' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    let position = self.current_position();
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                CompileError::parse(
                                    format!("Invalid supply value '{}'", val),
                                    self.current().line,
                                    self.current().col,
                                    self.source_line(self.current().line),
                                )
                            })?;
                            contract.supply = Some(IntField { value, position });
                            self.advance();
                        }
                        other => {
                            return Err(CompileError::parse(
                                format!("'supply' expects a number, got {}", describe_token(other)),
                                self.current().line,
                                self.current().col,
                                self.source_line(self.current().line),
                            ));
                        }
                    }
                }
                LexTokenType::Symbol => {
                    if let Some(ref existing) = contract.symbol {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'symbol' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    let position = self.current_position();
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::StringLit(val) => {
                            contract.symbol = Some(StringField {
                                value: val.clone(),
                                position,
                            });
                            self.advance();
                        }
                        other => {
                            return Err(CompileError::parse(
                                format!(
                                    "'symbol' expects a string, got {}",
                                    describe_token(other)
                                ),
                                self.current().line,
                                self.current().col,
                                self.source_line(self.current().line),
                            ));
                        }
                    }
                }
                LexTokenType::Decimals => {
                    if let Some(ref existing) = contract.decimals {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'decimals' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    let position = self.current_position();
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                CompileError::parse(
                                    format!("Invalid decimals value '{}'", val),
                                    self.current().line,
                                    self.current().col,
                                    self.source_line(self.current().line),
                                )
                            })?;
                            contract.decimals = Some(IntField { value, position });
                            self.advance();
                        }
                        other => {
                            return Err(CompileError::parse(
                                format!(
                                    "'decimals' expects a number, got {}",
                                    describe_token(other)
                                ),
                                self.current().line,
                                self.current().col,
                                self.source_line(self.current().line),
                            ));
                        }
                    }
                }
                LexTokenType::Mintable => {
                    if let Some(ref existing) = contract.mintable {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'mintable' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    contract.mintable = Some(FlagField {
                        position: self.current_position(),
                    });
                    self.advance();
                }
                LexTokenType::Burnable => {
                    if let Some(ref existing) = contract.burnable {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'burnable' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    contract.burnable = Some(FlagField {
                        position: self.current_position(),
                    });
                    self.advance();
                }
                LexTokenType::Capped => {
                    if let Some(ref existing) = contract.capped {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'capped' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    let position = self.current_position();
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                CompileError::parse(
                                    format!("Invalid capped value '{}'", val),
                                    self.current().line,
                                    self.current().col,
                                    self.source_line(self.current().line),
                                )
                            })?;
                            contract.capped = Some(IntField { value, position });
                            self.advance();
                        }
                        other => {
                            return Err(CompileError::parse(
                                format!(
                                    "'capped' expects a number, got {}",
                                    describe_token(other)
                                ),
                                self.current().line,
                                self.current().col,
                                self.source_line(self.current().line),
                            ));
                        }
                    }
                }
                LexTokenType::Owner => {
                    if let Some(ref existing) = contract.owner {
                        return Err(CompileError::parse(
                            format!(
                                "Duplicate 'owner' declaration — already set on line {}",
                                existing.position.line
                            ),
                            self.current().line,
                            self.current().col,
                            self.source_line(self.current().line),
                        ));
                    }
                    let position = self.current_position();
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::AddressLit { bytes, .. } => {
                            contract.owner = Some(AddressField {
                                value: *bytes,
                                position,
                            });
                            self.advance();
                        }
                        other => {
                            return Err(CompileError::parse(
                                format!(
                                    "'owner' expects an address, got {}",
                                    describe_token(other)
                                ),
                                self.current().line,
                                self.current().col,
                                self.source_line(self.current().line),
                            ));
                        }
                    }
                }
                LexTokenType::Identifier(word) => {
                    let word = word.clone();
                    let err = CompileError::parse(
                        format!("Unknown field '{}'", word),
                        self.current().line,
                        self.current().col,
                        self.source_line(self.current().line),
                    );
                    return Err(match suggest_closest(&word, LexTokenType::FIELD_KEYWORDS, 3) {
                        Some(suggestion) => {
                            err.with_suggestion(format!("Did you mean '{}'?", suggestion))
                        }
                        None => err,
                    });
                }
                LexTokenType::EOF => {
                    return Err(CompileError::parse(
                        "Expected '}' to close contract block",
                        self.current().line,
                        self.current().col,
                        self.source_line(self.current().line),
                    ));
                }
                _ => {
                    let desc = describe_token(&self.current().token_type);
                    return Err(CompileError::parse(
                        format!("Unexpected token {}", desc),
                        self.current().line,
                        self.current().col,
                        self.source_line(self.current().line),
                    ));
                }
            }
        }
        Ok(())
    }

    fn source_line(&self, line: usize) -> String {
        get_source_line(&self.source, line)
    }

    fn current_position(&self) -> Position {
        Position {
            line: self.current().line,
            col: self.current().col,
        }
    }

    fn current(&self) -> &LexToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &LexToken {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: &str) -> Result<&LexToken, CompileError> {
        let matches = match expected {
            "contract" => matches!(self.current().token_type, LexTokenType::Contract),
            "{" => matches!(self.current().token_type, LexTokenType::OpenBrace),
            "}" => matches!(self.current().token_type, LexTokenType::CloseBrace),
            "EOF" => matches!(self.current().token_type, LexTokenType::EOF),
            _ => false,
        };

        if !matches {
            let message = match expected {
                "contract" => "Expected 'contract' keyword".to_string(),
                "{" => "Expected '{' after contract name".to_string(),
                "}" => "Expected '}' to close contract block".to_string(),
                "EOF" => "Unexpected content after contract block".to_string(),
                _ => format!("Expected '{}'", expected),
            };
            return Err(CompileError::parse(
                message,
                self.current().line,
                self.current().col,
                self.source_line(self.current().line),
            ));
        }

        Ok(self.advance())
    }

    fn expect_identifier(&mut self) -> Result<(String, Position), CompileError> {
        match &self.current().token_type {
            LexTokenType::Identifier(s) => {
                let result = (
                    s.clone(),
                    Position {
                        line: self.current().line,
                        col: self.current().col,
                    },
                );
                self.advance();
                Ok(result)
            }
            _ => Err(CompileError::parse(
                "Missing contract name after 'contract'",
                self.current().line,
                self.current().col,
                self.source_line(self.current().line),
            )),
        }
    }
}

fn describe_token(token: &LexTokenType) -> String {
    match token {
        LexTokenType::StringLit(s) => format!("string \"{}\"", s),
        LexTokenType::IntegerLit(s) => format!("number {}", s),
        LexTokenType::AddressLit { raw, .. } => format!("address {}", raw),
        LexTokenType::Identifier(s) => format!("identifier '{}'", s),
        LexTokenType::OpenBrace => "'{'".to_string(),
        LexTokenType::CloseBrace => "'}'".to_string(),
        LexTokenType::Contract => "keyword 'contract'".to_string(),
        LexTokenType::Symbol => "keyword 'symbol'".to_string(),
        LexTokenType::Decimals => "keyword 'decimals'".to_string(),
        LexTokenType::Supply => "keyword 'supply'".to_string(),
        LexTokenType::Mintable => "keyword 'mintable'".to_string(),
        LexTokenType::Burnable => "keyword 'burnable'".to_string(),
        LexTokenType::Capped => "keyword 'capped'".to_string(),
        LexTokenType::Owner => "keyword 'owner'".to_string(),
        LexTokenType::EOF => "end of file".to_string(),
    }
}
