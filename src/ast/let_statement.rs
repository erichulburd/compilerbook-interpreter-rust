use super::expression::Expression;
use super::identifier::Identifier;
use super::statement_node::StatementNode;
use super::token_node::TokenNode;
use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Expression>,
}

impl TokenNode for LetStatement {
    fn token_type(&self) -> TokenType {
        TokenType::LET
    }

    fn token_literal(&self) -> String {
        String::from("LET")
    }

    fn string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.token_literal().to_string().as_str());
        s.push_str(" ");
        s.push_str(self.name.string().as_str());
        s.push_str(" = ");
        if self.value.is_some() {
            s.push_str(self.value.clone().unwrap().string().as_str());
        }
        s.push(';');
        s
    }
}

impl<'a> StatementNode for LetStatement {
    fn statement_node(&self) {}
}
