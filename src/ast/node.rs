use std::fmt;
pub trait Node: fmt::Display {
  fn value(&self) -> String;
}
