use crate::token::{Token, TokenType};
use super::identifier::Identifier;
use super::expression::Expression;
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub struct LetStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Option<Expression>,
}

impl<'a> TokenNode for LetStatement {
  fn token_literal(&self) -> TokenType {
      TokenType::LET
  }
}

impl<'a> StatementNode for LetStatement {
  fn statement_node(&self) {

  }
}
