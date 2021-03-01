use crate::token::TokenType;
use super::let_statement::LetStatement;
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub enum Statement {
  LetStatement(LetStatement)
}

impl<'a> TokenNode for Statement {
  fn token_literal(&self) -> TokenType {
    match self {
      Statement::LetStatement(let_statement) => {
        let_statement.token_literal()
      }
    }
  }
}

impl<'a> StatementNode for Statement {
  fn statement_node(&self) {
    match self {
      Statement::LetStatement(let_statement) => {
        let_statement.statement_node()
      }
    }
  }
}
