use super::statement_node::StatementNode;
use super::token_node::TokenNode;
use super::{
    expression_statement::ExpressionStatement, let_statement::LetStatement,
    return_statement::ReturnStatement,
};
use crate::token::TokenType;

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(ExpressionStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}

impl TokenNode for Statement {
    fn token_type(&self) -> TokenType {
        match self {
            Statement::ExpressionStatement(st) => st.token_type(),
            Statement::LetStatement(st) => st.token_type(),
            Statement::ReturnStatement(st) => st.token_type(),
        }
    }

    fn token_literal(&self) -> String {
        match self {
            Statement::ExpressionStatement(st) => st.token_literal(),
            Statement::LetStatement(st) => st.token_literal(),
            Statement::ReturnStatement(st) => st.token_literal(),
        }
    }

    fn string(&self) -> String {
        match self {
            Statement::ExpressionStatement(st) => st.string(),
            Statement::LetStatement(st) => st.string(),
            Statement::ReturnStatement(st) => st.string(),
        }
    }
}

impl<'a> StatementNode for Statement {
    fn statement_node(&self) {
        match self {
            Statement::ExpressionStatement(st) => st.statement_node(),
            Statement::LetStatement(st) => st.statement_node(),
            Statement::ReturnStatement(st) => st.statement_node(),
        }
    }
}
