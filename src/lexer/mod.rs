use crate::tokens::Token;
use std::str::Chars;

pub struct Lexer<'a> {
  input: Chars<'a>,
  cur: Option<char>,
  peek: Option<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut input = input.chars();
    let cur = input.next();
    let peek = input.next();
    Lexer { input, cur, peek }
  }

  fn next_char(&mut self) {
    self.cur = self.peek;
    self.peek = self.input.next();
  }

  fn eat_ws(&mut self) {
    loop {
      match self.cur {
        Some(' ') | Some('\n') | Some('\t') => self.next_char(),
        _ => break,
      }
    }
  }

  pub fn next_token(&mut self) -> Token {
    self.eat_ws();
    let token = match self.cur {
      Some('<') => match self.peek {
        Some('=') => {
          self.next_char();
          Token::SE
        }
        _ => Token::ST,
      },
      Some('>') => match self.peek {
        Some('=') => {
          self.next_char();
          Token::GE
        }
        _ => Token::GT,
      },
      Some('=') => match self.peek {
        Some('=') => {
          self.next_char();
          Token::EQ
        }
        _ => Token::ASSIGN,
      },
      Some('!') => match self.peek {
        Some('=') => {
          self.next_char();
          Token::NEQ
        }
        _ => Token::NOT,
      },
      Some('+') => Token::PLUS,
      Some('-') => Token::MINUS,
      Some('/') => Token::DIVIDE,
      Some('*') => Token::ASTERISK,
      Some(';') => Token::SEMICOLON,
      Some('(') => Token::LPAREN,
      Some(')') => Token::RPAREN,
      None => Token::EOF,
      _ => Token::UNDEFINED,
    };

    self.next_char();
    token
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn new_lexer() {
    let lexer = Lexer::new("");
    assert_eq!(lexer.cur, None);
    assert_eq!(lexer.peek, None);

    let lexer = Lexer::new("h");
    assert_eq!(lexer.cur, Some('h'));
    assert_eq!(lexer.peek, None);

    let lexer = Lexer::new("hallo");
    assert_eq!(lexer.cur, Some('h'));
    assert_eq!(lexer.peek, Some('a'));
  }

  #[test]
  fn next_token() {
    let mut lexer = Lexer::new("");

    let token = lexer.next_token();
    assert_eq!(token, Token::EOF);

    let mut lexer = Lexer::new("+ !=   >= -    /\t*\n += (   ) ");
    let mut token = lexer.next_token();
    assert_eq!(token, Token::PLUS);
    token = lexer.next_token();
    assert_eq!(token, Token::NEQ);
    token = lexer.next_token();
    assert_eq!(token, Token::GE);
    token = lexer.next_token();
    assert_eq!(token, Token::MINUS);
    token = lexer.next_token();
    assert_eq!(token, Token::DIVIDE);
    token = lexer.next_token();
    assert_eq!(token, Token::ASTERISK);
    token = lexer.next_token();
    assert_eq!(token, Token::PLUS);
    token = lexer.next_token();
    assert_eq!(token, Token::ASSIGN);
    token = lexer.next_token();
    assert_eq!(token, Token::LPAREN);
    token = lexer.next_token();
    assert_eq!(token, Token::RPAREN);
    token = lexer.next_token();
    assert_eq!(token, Token::EOF);
  }
}
