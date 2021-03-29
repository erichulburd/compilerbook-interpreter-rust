use crate::token::{Token, TokenType};

use super::{expression::Expression, token_node::TokenNode};

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl TokenNode for PrefixExpression {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = String::new();
        s.push_str("(");
        s.push_str(self.operator.as_str());
        if self.right.is_some() {
            s.push_str(self.right.clone().unwrap().string().as_str());
        }
        s.push_str(")");
        s
    }
}
