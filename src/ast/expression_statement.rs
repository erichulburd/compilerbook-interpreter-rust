use crate::token::{Token, TokenType};
use super::identifier::Identifier;
use super::expression::Expression;
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
  pub token: Token,
  pub value: Option<Expression>,
}

impl TokenNode for ExpressionStatement {
  fn token_type(&self) -> TokenType {
    self.token.token_type
  }

  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    if self.value.is_some() {
      return self.value.clone().unwrap().string();
    }
    String::from("")
  }

}

impl StatementNode for ExpressionStatement {
  fn statement_node(&self) {

  }
}
