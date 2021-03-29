use crate::token::{Token, TokenType};

use super::{expression::Expression, token_node::TokenNode};

#[derive(Debug, Clone)]
pub struct InfixExpression {
  pub token: Token,
  pub operator: String,
  pub left: Option<Box<Expression>>,
  pub right: Option<Box<Expression>>,
}

impl TokenNode for InfixExpression {
  fn token_type(&self) -> TokenType {
    self.token.token_type
  }

  fn token_literal(&self) -> String {
    self.token.literal.clone()
  }

  fn string(&self) -> String {
    let mut s = String::new();
    s.push_str("(");
    if self.left.is_some() {
      s.push_str(self.left.clone().unwrap().string().as_str());
    } else {
      s.push_str("_");
    }
    s.push_str(format!(" {} ", self.operator).as_str());
    if self.right.is_some() {
      s.push_str(self.right.clone().unwrap().string().as_str());
    } else {
      s.push_str("_");
    }
    s.push_str(")");
    s
  }
}
