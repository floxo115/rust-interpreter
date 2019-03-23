use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
  PLUS,
  MINUS,
  ASTERISK,
  DIVIDE,
  ASSIGN,
  SEMICOLON,
  LPAREN,
  RPAREN,
  NOT,

  EQ,
  NEQ,
  ST,
  SE,
  GT,
  GE,

  IDENTIFIER { value: &'a str },
  NUMBER { value: &'a str },

  LET,

  EOF,
  UNDEFINED,
}

impl<'a> Token<'a> {
  fn value(&self) -> String {
    match self {
      Token::PLUS => "+",
      Token::MINUS => "-",
      Token::DIVIDE => "/",
      Token::ASTERISK => "*",
      Token::ASSIGN => "=",
      Token::SEMICOLON => ";",
      Token::LPAREN => "(",
      Token::RPAREN => ")",
      Token::NOT => "!",
      Token::EQ => "==",
      Token::NEQ => "!=",
      Token::ST => "<",
      Token::SE => "<=",
      Token::GT => ">",
      Token::GE => ">=",
      Token::IDENTIFIER { value } | Token::NUMBER { value } => value,
      Token::LET => "let",
      Token::EOF | Token::UNDEFINED => "",
    }
    .to_string()
  }
}

impl<'a> Display for Token<'a> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "Token<'{:?}', '{}'>", self, self.value())
  }
}
