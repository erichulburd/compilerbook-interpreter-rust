use crate::token::TokenType;

pub trait TokenNode {
  fn token_literal(&self) -> TokenType;
  fn string(&self) -> String;
}

