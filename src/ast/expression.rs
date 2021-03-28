use super::{identifier::Identifier, integer_literal::IntegerLiteral, token_node::TokenNode};

#[derive(Debug, Clone)]
pub enum Expression {
  Identifier(Identifier),
  IntegerLiteral(IntegerLiteral),
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
    }
  }
}




