mod ast;
mod lexer;
mod parser;
mod tokens;
use lexer::Lexer;
use tokens::Token;
fn main() {
    let mut lexer = Lexer::new("a = 50 + 20");
    loop {
        let token = lexer.next_token();
        println!("{}", token);
        if let Token::EOF = token {
            break;
        }
    }
}
