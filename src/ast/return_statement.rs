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

impl TokenNode for ReturnStatement {
  fn token_type(&self) -> TokenType {
      TokenType::RETURN
  }

  fn token_literal(&self) -> String {
      String::from("RETURN")
  }


  fn string(&self) -> String {
    let mut s = String::new();
    s.push_str(self.token_literal().to_string().as_str());
    s.push_str(" ");
    if self.value.is_some() {
      s.push_str(self.value.clone().unwrap().string().as_str());
    }
    s.push(';');
    s
  }
}

impl<'a> StatementNode for ReturnStatement {
  fn statement_node(&self) {

  }
}
