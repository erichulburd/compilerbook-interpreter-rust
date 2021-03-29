use crate::token::TokenType;

pub trait TokenNode {
    fn token_type(&self) -> TokenType;
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}
