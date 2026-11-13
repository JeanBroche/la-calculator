pub trait Operand {
  fn get_value(&self) -> f64;
}

pub trait Operator {
  fn execute(&self) -> f64;
}

impl Operand for dyn Operator {
  fn get_value(&self) -> f64 {
      return self::Operator::execute(self)
  }
}