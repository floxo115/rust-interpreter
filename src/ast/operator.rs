use crate::ast::Node;
use std::fmt;

trait StrategyOperator {
  fn apply_operator(&self, nodes: &[Box<Node>]) -> String;
}

pub struct MultipleOperator {
  pub operator: Box<Fn(&[Box<Node>]) -> String>,
  pub nodes: Vec<Box<Node>>,
}

impl MultipleOperator {
  pub fn new_addition_operator() -> MultipleOperator {
    MultipleOperator {
      operator: Box::new(addition_strategy),
      nodes: vec![],
    }
  }
}

impl Node for MultipleOperator {
  fn value(&self) -> String {
    (self.operator)(&self.nodes)
  }
}

impl fmt::Display for MultipleOperator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value())
  }
}

fn addition_strategy(nodes: &[Box<Node>]) -> String {
  format!(
    "{}",
    nodes.iter().fold(0.0, |acc, el| {
      let value: f64 = el.value().parse().unwrap();
      acc + value
    })
  )
}
