use super::parser::Parser;
use crate::{ast::expression::Expression, ast::operators::Operator, token::TokenType};

impl<'a> Parser<'a> {
    pub fn parse_call_arguments(&mut self) -> Vec<Expression> {
        self.assert_current_token_type(TokenType::LPAREN);
        self.next_token();

        let mut arguments: Vec<Expression> = vec![];
        if self.current_token_is(TokenType::RPAREN) {
            return arguments;
        }

        let first_expression = self.parse_expression(Operator::LOWEST);

        arguments.push(first_expression.unwrap());

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let expression = self.parse_expression(Operator::LOWEST);
            arguments.push(expression.unwrap());
        }

        self.next_token();
        self.assert_current_token_type(TokenType::RPAREN);

        arguments
    }
}
