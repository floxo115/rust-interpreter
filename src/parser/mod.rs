use crate::lexer::Lexer;
use crate::tokens::Token;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  cur: Token<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut parser = Parser {
      lexer: Lexer::new(input),
      cur: Token::EOF,
    };
    parser.next_token();

    parser
  }

  fn next_token(&mut self) {
    self.cur = self.lexer.next_token();
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn next_token() {
    let mut parser = Parser::new("+ -  * /");
    assert_eq!(parser.cur, Token::PLUS);
    parser.next_token();
    assert_eq!(parser.cur, Token::MINUS);
    parser.next_token();
    assert_eq!(parser.cur, Token::ASTERISK);
    parser.next_token();
    assert_eq!(parser.cur, Token::DIVIDE);
    parser.next_token();
    assert_eq!(parser.cur, Token::EOF);
    parser.next_token();
    assert_eq!(parser.cur, Token::EOF);
  }
}
