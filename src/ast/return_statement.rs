use crate::token::{Token, TokenType};
use super::identifier::Identifier;
use super::expression::Expression;
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
  pub token: Token,
  pub value: Option<Expression>,
}

impl<'a> TokenNode for ReturnStatement {
  fn token_literal(&self) -> TokenType {
      TokenType::RETURN
  }
}

impl<'a> StatementNode for ReturnStatement {
  fn statement_node(&self) {

  }
}
