#![warn(unused_doc_comments)]

mod lex_tokens;
mod lexer;

use lexer::Lexer;

fn main() {
    let source = "contract MyToken { supply 1000000 }";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    println!("{:?}", tokens);
}
