use crate::calculator::core::core::{Operand, Operator};

struct Addition {
  operands: Vec<Box<dyn Operand>>
}

impl Operator for Addition {
  fn execute(&self) -> f64 {
      return 0.0;
  }
}