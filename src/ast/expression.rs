use std::path::Prefix;

use super::{identifier::Identifier, integer_literal::IntegerLiteral, prefix_expression::PrefixExpression, token_node::TokenNode};

#[derive(Debug, Clone)]
pub enum Expression {
  Identifier(Identifier),
  IntegerLiteral(IntegerLiteral),
  PrefixExpression(PrefixExpression),
}

impl Expression {
  pub fn string(&self) -> String {
    match self {
      Expression::Identifier(identifier) => {
        identifier.string()
      },
      Expression::IntegerLiteral(integer_literal) => {
        integer_literal.string()
      },
      Expression::PrefixExpression(prefix_expression) => {
        prefix_expression.string()
      },
    }
  }
}




