use super::{identifier::Identifier, token_node::TokenNode};

#[derive(Debug, Clone)]
pub enum Expression {
  Identifier(Identifier),
}

impl Expression {
  pub fn string(&self) -> String {
    match self {
      Expression::Identifier(identifier) => {
        identifier.string()
      },
    }
  }
}




