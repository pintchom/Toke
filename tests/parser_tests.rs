use toke::errors::CompileError;
use toke::lexer::Lexer;
use toke::parser::Parser;

fn parse(source: &str) -> Result<toke::ast::ContractNode, CompileError> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens, source);
    parser.parse()
}

#[test]
fn test_minimal_contract() {
    let contract = parse("contract MyToken { }").unwrap();
    assert_eq!(contract.name, "MyToken");
    assert!(contract.supply.is_none());
    assert!(contract.symbol.is_none());
    assert!(contract.decimals.is_none());
    assert!(contract.mintable.is_none());
    assert!(contract.burnable.is_none());
    assert!(contract.capped.is_none());
    assert!(contract.owner.is_none());
}

#[test]
fn test_full_contract() {
    let source = r#"
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
    let contract = parse(source).unwrap();
    assert_eq!(contract.name, "MyToken");
    assert_eq!(contract.symbol.unwrap().value, "MTK");
    assert_eq!(contract.decimals.unwrap().value, 18);
    assert_eq!(contract.supply.unwrap().value, 1000000);
    assert!(contract.mintable.is_some());
    assert!(contract.burnable.is_some());
    assert_eq!(contract.capped.unwrap().value, 5000000);
    assert!(contract.owner.is_some());
}

#[test]
fn test_missing_contract_keyword() {
    let result = parse("MyToken { supply 100 }");
    assert!(result.is_err());
}

#[test]
fn test_missing_contract_name() {
    let result = parse("contract { supply 100 }");
    assert!(result.is_err());
}

#[test]
fn test_missing_open_brace() {
    let result = parse("contract MyToken supply 100 }");
    assert!(result.is_err());
}

#[test]
fn test_missing_close_brace() {
    let result = parse("contract MyToken { supply 100");
    assert!(result.is_err());
}

#[test]
fn test_duplicate_supply() {
    let result = parse("contract MyToken { supply 100 supply 200 }");
    assert!(result.is_err());
}

#[test]
fn test_duplicate_mintable() {
    let result = parse("contract MyToken { mintable mintable }");
    assert!(result.is_err());
}

#[test]
fn test_wrong_type_for_supply() {
    let result = parse(r#"contract MyToken { supply "notanumber" }"#);
    assert!(result.is_err());
}

#[test]
fn test_wrong_type_for_symbol() {
    let result = parse("contract MyToken { symbol 123 }");
    assert!(result.is_err());
}

#[test]
fn test_extra_content_after_contract() {
    let result = parse("contract A { } contract B { }");
    assert!(result.is_err());
}

#[test]
fn test_comment_before_contract() {
    let contract = parse("# this is a comment\ncontract MyToken { }").unwrap();
    assert_eq!(contract.name, "MyToken");
}

// --- error structure ---

#[test]
fn test_parser_error_kind() {
    let err = parse("MyToken { supply 100 }").unwrap_err();
    assert_eq!(err.kind, toke::errors::ErrorKind::ParseError);
}

#[test]
fn test_typo_suggests_correction() {
    let err = parse("contract MyToken { supplly 100 }").unwrap_err();
    assert!(err.suggestion.is_some());
    assert!(err.suggestion.unwrap().contains("supply"));
}

#[test]
fn test_unknown_field_no_suggestion() {
    let err = parse("contract MyToken { foo 100 }").unwrap_err();
    assert!(err.suggestion.is_none());
}

#[test]
fn test_parser_error_source_line() {
    let source = "contract MyToken {\n  supplly 100\n}";
    let err = parse(source).unwrap_err();
    assert_eq!(err.source_line, "  supplly 100");
}

#[test]
fn test_duplicate_field_references_original_line() {
    let source = "contract MyToken {\n  supply 100\n  supply 200\n}";
    let err = parse(source).unwrap_err();
    assert!(err.message.contains("already set on line"));
}

#[test]
fn test_wrong_type_describes_actual() {
    let err = parse(r#"contract MyToken { supply "notanumber" }"#).unwrap_err();
    assert!(err.message.contains("string"));
}
