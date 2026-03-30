#![warn(unused_doc_comments)]

use toke::lexer::Lexer;

fn main() {
    let source = "contract MyToken { supply 1000000 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    println!("Lexed {} tokens", tokens.len());
}
