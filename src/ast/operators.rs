use core::fmt;
use std::cmp::Ordering;

use crate::token::TokenType;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Operator {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)”
}

impl PartialOrd for Operator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let precedence_self = get_operator_precedence(self);
        let precedence_other = get_operator_precedence(other);
        Some(precedence_self.cmp(&precedence_other))
    }
}

fn get_operator_precedence(operator: &Operator) -> i8 {
    match operator {
        Operator::LOWEST => 1,
        Operator::EQUALS => 2,
        Operator::LESSGREATER => 3,
        Operator::SUM => 4,
        Operator::PRODUCT => 5,
        Operator::PREFIX => 6,
        Operator::CALL => 7,
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::LOWEST => write!(f, "lowest"),
            Operator::EQUALS => write!(f, "equals"),
            Operator::LESSGREATER => write!(f, "lesser greater"),
            Operator::SUM => write!(f, "sum"),
            Operator::PRODUCT => write!(f, "product"),
            Operator::PREFIX => write!(f, "prefix"),
            Operator::CALL => write!(f, "call"),
        }
    }
}

pub fn get_token_type_operator_precedence(token_type: TokenType) -> Operator {
    match token_type {
        TokenType::EQ => Operator::EQUALS,
        TokenType::NotEq => Operator::EQUALS,
        TokenType::LT => Operator::LESSGREATER,
        TokenType::GT => Operator::LESSGREATER,
        TokenType::PLUS => Operator::SUM,
        TokenType::MINUS => Operator::SUM,
        TokenType::SLASH => Operator::PRODUCT,
        TokenType::ASTERISK => Operator::PRODUCT,
        TokenType::LPAREN => Operator::CALL,
        _ => Operator::LOWEST,
    }
}
