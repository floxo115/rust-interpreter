mod ast;
mod lexer;
mod parser;
mod tokens;
use crate::ast::node::Node;
use crate::parser::Parser;
use std::io::BufRead;
use std::io::Read;
use tokens::Token;

use std::io;
fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut parser = Parser::new(&input);
    let prog = parser.parse_program();

    let val = prog.value();

    println!("{}", val);
}
