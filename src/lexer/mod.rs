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

  fn get_token_with_peek(&mut self) -> Option<Token> {
    let token = match self.cur {
      Some('<') => match self.peek {
        Some('=') => {
          self.next_char();
          Some(Token::SE)
        }
        _ => Some(Token::ST),
      },
      Some('>') => match self.peek {
        Some('=') => {
          self.next_char();
          Some(Token::GE)
        }
        _ => Some(Token::GT),
      },
      Some('=') => match self.peek {
        Some('=') => {
          self.next_char();
          Some(Token::EQ)
        }
        _ => Some(Token::ASSIGN),
      },
      Some('!') => match self.peek {
        Some('=') => {
          self.next_char();
          Some(Token::NEQ)
        }
        _ => Some(Token::NOT),
      },
      _ => None,
    };

    token
  }

  fn get_token_without_peek(&mut self) -> Option<Token> {
    let token = match self.cur {
      Some('+') => Token::PLUS,
      Some('-') => Token::MINUS,
      Some('/') => Token::DIVIDE,
      Some('*') => Token::ASTERISK,
      Some(';') => Token::SEMICOLON,
      Some('(') => Token::LPAREN,
      Some(')') => Token::RPAREN,
      None => Token::EOF,
      _ => return None,
    };

    Some(token)
  }

  fn get_token_from_value(&mut self) -> Option<Token> {
    fn get_identifier_value(lexer: &mut Lexer) -> String {
      let mut value = String::new();
      while let Some(cur) = lexer.cur {
        if !cur.is_alphabetic() {
          break;
        }

        value.push(cur);
        lexer.next_char();
      }

      value
    }

    fn get_number_value(lexer: &mut Lexer) -> String {
      fn get_numerics(lexer: &mut Lexer, mut value: String) -> String {
        while let Some(cur) = lexer.cur {
          if !cur.is_numeric() {
            break;
          }

          value.push(cur);
          lexer.next_char();
        }
        value
      }

      let mut value = String::new();
      // before decimal point
      value = get_numerics(lexer, value);
      // if there is a decimal point
      if let Some('.') = lexer.cur {
        value.push('.');
        lexer.next_char();
        value = get_numerics(lexer, value);
      }
      value
    }

    let token = match self.cur {
      Some(first_char) if first_char.is_alphabetic() => {
        let value = get_identifier_value(self);
        // TODO keyword check
        if let Some(token) = Token::get_keyword(&value) {
          token
        } else {
          Token::IDENTIFIER { value }
        }
      }
      Some(first_char) if first_char.is_numeric() => {
        let value = get_number_value(self);
        Token::NUMBER { value }
      }
      _ => return None,
    };

    Some(token)
  }

  pub fn next_token(&mut self) -> Token {
    self.eat_ws();

    let token;
    if let Some(created_token) = self.get_token_with_peek() {
      token = created_token;
      self.next_char();
    } else if let Some(created_token) = self.get_token_without_peek() {
      token = created_token;
      self.next_char();
    } else if let Some(created_token) = self.get_token_from_value() {
      token = created_token;
    } else {
      token = Token::UNDEFINED;
    }

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

    let mut lexer = Lexer::new("  let 30.5 15 hallo + != !   >= -    /\t*\n += (   ) 60 helo");
    let mut token = lexer.next_token();
    assert_eq!(Token::LET, token);
    token = lexer.next_token();
    assert_eq!(
      token,
      Token::NUMBER {
        value: "30.5".to_string()
      }
    );
    token = lexer.next_token();
    assert_eq!(
      token,
      Token::NUMBER {
        value: "15".to_string()
      }
    );
    token = lexer.next_token();
    assert_eq!(
      token,
      Token::IDENTIFIER {
        value: "hallo".to_string()
      }
    );
    token = lexer.next_token();
    assert_eq!(token, Token::PLUS);
    token = lexer.next_token();
    assert_eq!(token, Token::NEQ);
    token = lexer.next_token();
    assert_eq!(token, Token::NOT);
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
    assert_eq!(
      token,
      Token::NUMBER {
        value: "60".to_string()
      }
    );
    token = lexer.next_token();
    assert_eq!(
      token,
      Token::IDENTIFIER {
        value: "helo".to_string()
      }
    );
    token = lexer.next_token();
    assert_eq!(token, Token::EOF);
  }
}
