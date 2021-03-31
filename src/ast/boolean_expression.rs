use super::{statement_node::StatementNode, token_node::TokenNode};
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub token: Token,
    pub value: bool,
}

impl TokenNode for BooleanExpression {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

impl StatementNode for BooleanExpression {
    fn statement_node(&self) {}
}
