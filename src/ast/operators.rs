use std::{cmp::Ordering, collections::HashMap};

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

