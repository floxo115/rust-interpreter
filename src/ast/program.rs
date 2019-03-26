use crate::ast::node::*;
use std::fmt;

pub struct Program {
  pub nodes: Vec<Box<dyn Node>>,
}

impl Node for Program {
  fn value(&self) -> String {
    let mut value = "0".to_string();
    for node in &self.nodes {
      value = node.value();
    }

    value
  }
}

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut res = String::new();
    for node in &self.nodes {
      let node_repr = format!("{}", node);
      res.push_str(&node_repr)
    }

    write!(f, "{}", res)
  }
}
