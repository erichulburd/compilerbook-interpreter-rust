use crate::token::TokenType;
use super::{let_statement::LetStatement, return_statement::ReturnStatement};
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
}

impl<'a> TokenNode for Statement {
  fn token_literal(&self) -> TokenType {
    match self {
      Statement::LetStatement(st) => {
        st.token_literal()
      },
      Statement::ReturnStatement(st) => {
        st.token_literal()
      }
    }
  }
}

impl<'a> StatementNode for Statement {
  fn statement_node(&self) {
    match self {
      Statement::LetStatement(st) => {
        st.statement_node()
      },
      Statement::ReturnStatement(st) => {
        st.statement_node()
      },
    }
  }
}
