use crate::token::{Token, TokenType};

use super::token_node::TokenNode;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl TokenNode for IntegerLiteral {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.token.literal.clone()
    }
}
