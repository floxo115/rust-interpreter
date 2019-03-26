use crate::ast::node::*;
use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Number {
  pub value: f64,
}

impl Node for Number {
  fn value(&self) -> String {
    format!("{}", self.value)
  }
}
impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}
