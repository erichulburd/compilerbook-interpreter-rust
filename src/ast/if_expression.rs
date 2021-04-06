use crate::token::{Token, TokenType};

use super::{block_statement::BlockStatement, expression::Expression, token_node::TokenNode};

#[derive(Debug, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<Expression>,
    pub consequence: Box<BlockStatement>,
    pub alternative: Option<Box<BlockStatement>>,
}

impl TokenNode for IfExpression {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = format!("if {} {}", self.condition.string(), self.consequence.string());

        if self.alternative.is_some() {
            let alternative = self.alternative.clone().unwrap().string();
            s.push_str(format!(" else {}", alternative).as_str());
        }

        s
    }
}
