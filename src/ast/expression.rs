use std::path::Prefix;

use super::{
    boolean_expression::BooleanExpression, identifier::Identifier,
    infix_expression::InfixExpression, integer_literal::IntegerLiteral,
    prefix_expression::PrefixExpression, token_node::TokenNode,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    Boolean(BooleanExpression),
    InfixExpression(InfixExpression),
    PrefixExpression(PrefixExpression),
}

impl Expression {
    pub fn string(&self) -> String {
        match self {
            Expression::Identifier(identifier) => identifier.string(),
            Expression::IntegerLiteral(integer_literal) => integer_literal.string(),
            Expression::Boolean(boolean_expression) => boolean_expression.string(),
            Expression::PrefixExpression(prefix_expression) => prefix_expression.string(),
            Expression::InfixExpression(infix_expression) => infix_expression.string(),
        }
    }
}
