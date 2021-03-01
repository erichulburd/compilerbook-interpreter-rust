use crate::token::{Token, TokenType};
use super::token_node::{TokenNode};

#[derive(Debug, Clone)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl<'a> TokenNode for Identifier {
  fn token_literal(&self) -> TokenType {
      TokenType::EQ
  }
}
