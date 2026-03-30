use crate::ast::{AddressField, ContractNode, FlagField, IntField, Position, StringField};
use crate::lex_tokens::{LexToken, LexTokenType};

pub struct Parser {
    tokens: Vec<LexToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<LexToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<ContractNode, String> {
        //confirm that we start with -- contract ....
        self.expect("contract")?;

        //expect identifier (contract name) -- contract xxx ...
        let (name, name_position) = self.expect_identifier()?;

        //expect open brace -- contract xxx { ...
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

        //parse fields until CloseBrace
        self.parse_fields(&mut contract)?;

        self.expect("}")?;
        self.expect("EOF")?;

        Ok(contract)
    }

    fn parse_fields(&mut self, contract: &mut ContractNode) -> Result<(), String> {
        // Loop until CloseBrace or EOF
        // Match current token:
        //   Supply   → expect IntegerLit, set contract.supply
        //   Symbol   → expect StringLit, set contract.symbol
        //   Decimals → expect IntegerLit, set contract.decimals
        //   Capped   → expect IntegerLit, set contract.capped
        //   Owner    → expect AddressLit, set contract.owner
        //   CloseBrace → break
        //   other    → error "Unknown field"
        //
        // Check for duplicates before setting any field

        while !matches!(self.current().token_type, LexTokenType::CloseBrace) {
            match &self.current().token_type {
                LexTokenType::Supply => {
                    if contract.supply.is_some() {
                        return Err(format!(
                            "Duplicate 'supply' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                format!(
                                    "Invalid supply value at line {}, column {}",
                                    self.current().line,
                                    self.current().col
                                )
                            })?;
                            contract.supply = Some(IntField { value, position });
                            self.advance();
                        }
                        _ => {
                            return Err(format!(
                                "Expected type number after 'supply' at line {}, column {}",
                                self.current().line,
                                self.current().col
                            ));
                        }
                    }
                }
                LexTokenType::Symbol => {
                    if contract.symbol.is_some() {
                        return Err(format!(
                            "Duplicate 'symbol' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::StringLit(val) => {
                            contract.symbol = Some(StringField {
                                value: val.clone(),
                                position,
                            });
                            self.advance();
                        }
                        _ => {
                            return Err(format!(
                                "Expected string literal at line {}, column {}",
                                self.current().line,
                                self.current().col
                            ));
                        }
                    }
                }
                LexTokenType::Decimals => {
                    if contract.decimals.is_some() {
                        return Err(format!(
                            "Duplicate 'decimals' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                format!(
                                    "Invalid decimals value at line {}, column {}",
                                    self.current().line,
                                    self.current().col
                                )
                            })?;
                            contract.decimals = Some(IntField { value, position });
                            self.advance();
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after 'decimals' at line {}, column {}",
                                self.current().line,
                                self.current().col
                            ));
                        }
                    }
                }
                LexTokenType::Mintable => {
                    if contract.mintable.is_some() {
                        return Err(format!(
                            "Duplicate 'mintable' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    contract.mintable = Some(FlagField { position });
                    self.advance();
                }
                LexTokenType::Burnable => {
                    if contract.burnable.is_some() {
                        return Err(format!(
                            "Duplicate 'burnable' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    contract.burnable = Some(FlagField { position });
                    self.advance();
                }
                LexTokenType::Capped => {
                    if contract.capped.is_some() {
                        return Err(format!(
                            "Duplicate 'capped' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::IntegerLit(val) => {
                            let value = val.parse::<u64>().map_err(|_| {
                                format!(
                                    "Invalid capped value at line {}, column {}",
                                    self.current().line,
                                    self.current().col
                                )
                            })?;
                            contract.capped = Some(IntField { value, position });
                            self.advance();
                        }
                        _ => {
                            return Err(format!(
                                "Expected number after 'capped' at line {}, column {}",
                                self.current().line,
                                self.current().col
                            ));
                        }
                    }
                }
                LexTokenType::Owner => {
                    if contract.owner.is_some() {
                        return Err(format!(
                            "Duplicate 'owner' at line {}, column {}",
                            self.current().line,
                            self.current().col
                        ));
                    }
                    let position = Position {
                        line: self.current().line,
                        col: self.current().col,
                    };
                    self.advance();
                    match &self.current().token_type {
                        LexTokenType::AddressLit { bytes, .. } => {
                            contract.owner = Some(AddressField {
                                value: *bytes,
                                position,
                            });
                            self.advance();
                        }
                        _ => {
                            return Err(format!(
                                "Expected address after 'owner' at line {}, column {}",
                                self.current().line,
                                self.current().col
                            ));
                        }
                    }
                }
                LexTokenType::CloseBrace => break,
                LexTokenType::EOF => {
                    return Err(format!(
                        "Expected '}}' to close contract block at line {}, column {}",
                        self.current().line,
                        self.current().col
                    ));
                }
                _ => {
                    return Err(format!(
                        "Unknown field at line {}, column {}",
                        self.current().line,
                        self.current().col
                    ));
                }
            }
        }
        Ok(())
    }

    fn current(&self) -> &LexToken {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &LexToken {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }

    fn expect(&mut self, expected: &str) -> Result<&LexToken, String> {
        let matches = match expected {
            "contract" => matches!(self.current().token_type, LexTokenType::Contract),
            "{" => matches!(self.current().token_type, LexTokenType::OpenBrace),
            "}" => matches!(self.current().token_type, LexTokenType::CloseBrace),
            "EOF" => matches!(self.current().token_type, LexTokenType::EOF),
            _ => false,
        };

        if !matches {
            return Err(format!(
                "Expected '{}' at line {}, column {}",
                expected,
                self.current().line,
                self.current().col
            ));
        }

        Ok(self.advance())
    }
    fn expect_identifier(&mut self) -> Result<(String, Position), String> {
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
            _ => Err(format!(
                "Expected identifier at line {}, column {}",
                self.current().line,
                self.current().col
            )),
        }
    }
}
