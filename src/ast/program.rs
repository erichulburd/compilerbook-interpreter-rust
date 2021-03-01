use crate::token::TokenType;
use super::statement::Statement;
use super::token_node::TokenNode;

pub struct Program {
  pub statements: Vec<Statement>
}

impl<'a> TokenNode for Program {
  fn token_literal(&self) -> TokenType {
      if self.statements.len() > 0 {
        self.statements[0].token_literal()
      } else {
        TokenType::EOF
      }
  }
}
