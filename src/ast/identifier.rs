use super::token_node::TokenNode;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl TokenNode for Identifier {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        self.value.clone()
    }
}
