use std::{cmp::Ordering};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Operator {
  LOWEST,
  EQUALS,      // ==
  LESSGREATER, // > or <
  SUM,         // +
  PRODUCT,     // *
  PREFIX,      // -X or !X
  CALL,        // myFunction(X)”
}

impl PartialOrd for Operator {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let precedence_self = get_operator_precedence(self);
    let precedence_other = get_operator_precedence(other);
    Some(precedence_self.cmp(&precedence_other))
  }
}

fn get_operator_precedence(operator: &Operator) -> i8 {
  match operator {
    Operator::LOWEST => 1,
    Operator::EQUALS => 2,
    Operator::LESSGREATER => 3,
    Operator::SUM => 4,
    Operator::PRODUCT => 5,
    Operator::PREFIX => 6,
    Operator::CALL => 7,
  }
}

fn get_operator_string(operator: &Operator) -> String {
  match operator {
    Operator::LOWEST => String::from("lowest"),
    Operator::EQUALS => String::from("equals"),
    Operator::LESSGREATER => String::from("lesser greater"),
    Operator::SUM => String::from("sum"),
    Operator::PRODUCT => String::from("product"),
    Operator::PREFIX => String::from("prefix"),
    Operator::CALL => String::from("call"),
  }
}
