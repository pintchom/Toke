use toke::errors::{get_source_line, suggest_closest, CompileError, ErrorKind};

// --- suggest_closest (fuzzy matching) ---

#[test]
fn test_suggest_close_typo() {
    let result = suggest_closest("supplly", &["supply", "symbol", "decimals"], 3);
    assert_eq!(result, Some("supply".to_string()));
}

#[test]
fn test_suggest_single_char_off() {
    let result = suggest_closest("symbo", &["supply", "symbol", "decimals"], 3);
    assert_eq!(result, Some("symbol".to_string()));
}

#[test]
fn test_suggest_no_close_match() {
    let result = suggest_closest("foo", &["supply", "symbol", "decimals"], 3);
    assert_eq!(result, None);
}

#[test]
fn test_suggest_exact_match_returns_none() {
    let result = suggest_closest("supply", &["supply", "symbol", "decimals"], 3);
    assert_eq!(result, None);
}

#[test]
fn test_suggest_picks_closest() {
    let result = suggest_closest("suppl", &["supply", "symbol"], 3);
    assert_eq!(result, Some("supply".to_string()));
}

// --- get_source_line ---

#[test]
fn test_get_source_line_first() {
    let source = "contract MyToken {\n  supply 1000\n}";
    assert_eq!(get_source_line(source, 1), "contract MyToken {");
}

#[test]
fn test_get_source_line_middle() {
    let source = "contract MyToken {\n  supply 1000\n}";
    assert_eq!(get_source_line(source, 2), "  supply 1000");
}

#[test]
fn test_get_source_line_out_of_bounds() {
    assert_eq!(get_source_line("one line", 99), "");
}

#[test]
fn test_get_source_line_zero() {
    assert_eq!(get_source_line("one line", 0), "one line");
}

// --- CompileError Display ---

#[test]
fn test_display_error_with_source_line() {
    let err = CompileError::lexer("Unexpected character '@'", 3, 10, "  symbol @invalid");
    let output = format!("{}", err);
    assert!(output.contains("Error [line 3, col 10]: Unexpected character '@'"));
    assert!(output.contains("3 | "));
    assert!(output.contains("symbol @invalid"));
    assert!(output.contains("^"));
}

#[test]
fn test_display_error_with_suggestion() {
    let err = CompileError::parse("Unknown field 'supplly'", 2, 3, "  supplly 1000000")
        .with_suggestion("Did you mean 'supply'?");
    let output = format!("{}", err);
    assert!(output.contains("Unknown field 'supplly'"));
    assert!(output.contains("Did you mean 'supply'?"));
}

#[test]
fn test_display_warning_label() {
    let err = CompileError::warning("Decimals is set to 6", 3, 3, "  decimals 6");
    let output = format!("{}", err);
    assert!(output.starts_with("Warning"));
    assert!(!output.starts_with("Error"));
}

#[test]
fn test_display_without_source_line() {
    let err = CompileError::semantic("Missing required field 'supply'", 1, 1, "");
    let output = format!("{}", err);
    assert!(output.contains("Error [line 1, col 1]"));
    assert!(!output.contains("|"));
}

// --- ErrorKind constructors ---

#[test]
fn test_error_kind_constructors() {
    assert_eq!(
        CompileError::lexer("msg", 1, 1, "").kind,
        ErrorKind::LexerError
    );
    assert_eq!(
        CompileError::parse("msg", 1, 1, "").kind,
        ErrorKind::ParseError
    );
    assert_eq!(
        CompileError::semantic("msg", 1, 1, "").kind,
        ErrorKind::SemanticError
    );
    assert_eq!(
        CompileError::warning("msg", 1, 1, "").kind,
        ErrorKind::Warning
    );
}
