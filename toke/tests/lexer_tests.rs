use toke::lex_tokens::LexTokenType;
use toke::lexer::Lexer;

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

#[test]
fn test_full_contract() {
    let source = r#"
# Full token contract
contract MyToken {
    symbol "MTK"
    decimals 18
    supply 1000000
    mintable
    burnable
    capped 5000000
    owner 0x1234567890abcdef1234567890abcdef12345678
}
"#;
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 17);
    assert!(matches!(tokens[0].token_type, LexTokenType::Contract));
    assert!(matches!(tokens[1].token_type, LexTokenType::Identifier(ref s) if s == "MyToken"));
    assert!(matches!(tokens[2].token_type, LexTokenType::OpenBrace));
    assert!(matches!(tokens[3].token_type, LexTokenType::Symbol));
    assert!(matches!(tokens[4].token_type, LexTokenType::StringLit(ref s) if s == "MTK"));
    assert!(matches!(tokens[5].token_type, LexTokenType::Decimals));
    assert!(matches!(tokens[6].token_type, LexTokenType::IntegerLit(ref s) if s == "18"));
    assert!(matches!(tokens[7].token_type, LexTokenType::Supply));
    assert!(matches!(tokens[8].token_type, LexTokenType::IntegerLit(ref s) if s == "1000000"));
    assert!(matches!(tokens[9].token_type, LexTokenType::Mintable));
    assert!(matches!(tokens[10].token_type, LexTokenType::Burnable));
    assert!(matches!(tokens[11].token_type, LexTokenType::Capped));
    assert!(matches!(tokens[12].token_type, LexTokenType::IntegerLit(ref s) if s == "5000000"));
    assert!(matches!(tokens[13].token_type, LexTokenType::Owner));
    assert!(matches!(tokens[14].token_type, LexTokenType::AddressLit { ref raw, .. } if raw == "0x1234567890abcdef1234567890abcdef12345678"));
    assert!(matches!(tokens[15].token_type, LexTokenType::CloseBrace));
    assert!(matches!(tokens[16].token_type, LexTokenType::EOF));
}

#[test]
fn test_comment_skipped() {
    let mut lexer = Lexer::new("# this is a comment\ncontract X { }");
    let tokens = lexer.tokenize().unwrap();

    assert!(matches!(tokens[0].token_type, LexTokenType::Contract));
}

#[test]
fn test_invalid_number() {
    let mut lexer = Lexer::new("contract X { supply 123abc }");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_unterminated_string() {
    let mut lexer = Lexer::new("contract X { symbol \"MTK }");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_unexpected_character() {
    let mut lexer = Lexer::new("contract X { @ }");
    let result = lexer.tokenize();
    assert!(result.is_err());
}

#[test]
fn test_short_address() {
    let mut lexer = Lexer::new("contract X { owner 0x1234 }");
    let result = lexer.tokenize();
    assert!(result.is_err());
}
