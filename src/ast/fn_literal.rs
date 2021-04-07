use super::{block_statement::BlockStatement, token_node::TokenNode};
use super::identifier::Identifier;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct FnLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: Option<BlockStatement>,
}

impl TokenNode for FnLiteral {
    fn token_type(&self) -> TokenType {
        self.token.token_type
    }

    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn string(&self) -> String {
        let mut s = format!("{} (", self.token_literal());
        let mut params: Vec<String> = vec![];
        for parameter in self.parameters.iter() {
          params.push(parameter.string());
        }
        s.push_str(params.join(", ").as_str());
        s.push_str(") ");
        if self.body.is_some() {
          s.push_str(self.body.clone().unwrap().string().as_str());
        }
        s
    }
}
