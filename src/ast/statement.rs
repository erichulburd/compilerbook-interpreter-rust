use crate::token::TokenType;
use super::{
  let_statement::LetStatement,
  return_statement::ReturnStatement,
  expression_statement::ExpressionStatement,
};
use super::statement_node::StatementNode;
use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub enum Statement {
  ExpressionStatement(ExpressionStatement),
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
}

impl<'a> TokenNode for Statement {
  fn token_literal(&self) -> TokenType {
    match self {
      Statement::ExpressionStatement(st) => {
        st.token_literal()
      },
      Statement::LetStatement(st) => {
        st.token_literal()
      },
      Statement::ReturnStatement(st) => {
        st.token_literal()
      }
    }
  }

  fn string(&self) -> String {
    match self {
      Statement::ExpressionStatement(st) => {
        st.string()
      },
      Statement::LetStatement(st) => {
        st.string()
      },
      Statement::ReturnStatement(st) => {
        st.string()
      },
    }
  }
}

impl<'a> StatementNode for Statement {
  fn statement_node(&self) {
    match self {
      Statement::ExpressionStatement(st) => {
        st.statement_node()
      },
      Statement::LetStatement(st) => {
        st.statement_node()
      },
      Statement::ReturnStatement(st) => {
        st.statement_node()
      },
    }
  }
}
