use crate::ast;
use crate::lexer::Lexer;
use crate::tokens::Token;

pub struct Parser<'a> {
  lexer: Lexer<'a>,
  cur: Token,
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

  pub fn parse_program(&mut self) -> ast::Program {
    let mut program = ast::Program { nodes: vec![] };

    while self.cur != Token::EOF {
      let node = self.parse_node();
      program.nodes.push(node);
    }

    program
  }

  fn parse_node(&mut self) -> Box<dyn ast::Node> {
    match &self.cur {
      Token::NUMBER { .. } => self.parse_number(),
      Token::PLUS => self.parse_operator(),
      _ => unimplemented!(),
    }
  }

  fn parse_operator(&mut self) -> Box<dyn ast::Node> {
    self.next_token();
    if self.cur != Token::LPAREN {
      panic!("expected LPAREN");
    }
    self.next_token();
    let mut operator = ast::MultipleOperator::new_addition_operator();

    while self.cur != Token::RPAREN {
      if self.cur == Token::EOF {
        panic!("expected RPAREN but got EOF")
      }
      let node = self.parse_node();
      println!("{}", self.cur);
      operator.nodes.push(node);
    }

    self.next_token();

    Box::new(operator)
  }

  fn parse_number(&mut self) -> Box<ast::Number> {
    let parsed_val = self.cur.value().parse().unwrap();
    self.next_token();
    Box::new(ast::Number { value: parsed_val })
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::ast::Node as _;
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

  #[test]
  fn parse_program() {
    let mut parser = Parser::new("50.1");
    let prog = parser.parse_program();
    let number = ast::Number { value: 50.1 };
    assert_eq!(prog.value(), number.value());

    let mut parser = Parser::new("+(50 40 20 )");

    let prog = parser.parse_program();
    assert_eq!(prog.value(), "110");

    let mut parser = Parser::new("+( 10 20 30.5)");
    let prog = parser.parse_program();
    assert_eq!(prog.value(), "60.5");
  }
}
