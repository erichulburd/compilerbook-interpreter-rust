use super::{statement::Statement, statement_node::StatementNode};
use super::token_node::TokenNode;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl TokenNode for BlockStatement {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = String::new();
        for statement in self.statements.iter() {
            s.push_str(statement.string().as_str());
        }
        s
    }
}

impl StatementNode for BlockStatement {
    fn statement_node(&self) {}
}
