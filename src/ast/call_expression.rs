use super::expression::Expression;
use super::token_node::TokenNode;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Expression,
    pub arguments: Vec<Expression>,
}

impl TokenNode for CallExpression {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = format!("{}(", self.function.string());
        let arguments = self
            .arguments
            .iter()
            .map(|exp| exp.string())
            .collect::<Vec<_>>();
        s.push_str(arguments.join(", ").as_str());
        s.push(')');
        s
    }
}
