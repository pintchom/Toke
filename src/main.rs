use toke::analyzer;
use toke::codegen;
use toke::lexer::Lexer;
use toke::parser::Parser;

fn main() {
    let source = r#"
contract CoolCoin {
    symbol "COOL"
    decimals 18
    supply 5000000
    mintable
    capped 10000000
}
"#;

    println!("Source: {}\n", source);

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    println!("Lexed {} tokens", tokens.len());

    let mut parser = Parser::new(tokens, source);
    let contract = parser.parse().unwrap();
    println!("Parsed contract: {}", contract.name);

    let result = analyzer::analyze(&contract, source);
    if !result.errors.is_empty() {
        for err in &result.errors {
            eprintln!("{}", err);
        }
        std::process::exit(1);
    }
    for warn in &result.warnings {
        eprintln!("{}", warn);
    }
    println!("Analysis: 0 errors, {} warnings", result.warnings.len());

    let bytecode = codegen::generate(&contract).unwrap();
    println!("\nBytecode ({} bytes):", bytecode.len());
    println!("0x{}", hex::encode(&bytecode));
}
