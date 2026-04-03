use toke::lexer::Lexer;
use toke::parser::Parser;

fn main() {
    let source = "contract MyToken { supply 1000000 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens, source);
    let contract = parser.parse().unwrap();
    println!("Parsed contract: {}", contract.name);
}
