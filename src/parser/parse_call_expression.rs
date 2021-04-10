use super::parser::Parser;
use crate::{
    ast::expression::Expression,
    ast::{call_expression::CallExpression},
};

impl<'a> Parser<'a> {
    pub fn parse_call_expression(&mut self, function: Expression) -> Expression {
        let token = self.current_token.clone().unwrap();
        self.next_token();
        let arguments = self.parse_call_arguments();

        let call_expression = CallExpression {
            token: token,
            function: function,
            arguments: arguments,
        };

        Expression::CallExpression(Box::new(call_expression))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expression;
    use crate::ast::program::Program;
    use crate::ast::statement::Statement;
    use crate::ast::token_node::TokenNode;
    use crate::lexer::Lexer;

    use super::Parser;

    #[test]
    fn parse_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program: Program = p.parse_program();
        assert_eq!(0, p.errors.len());
        assert_eq!(
            1,
            program.statements.len(),
            "unexpected number of statements parsed"
        );

        let stmt = match program.statements[0].clone() {
            Statement::ExpressionStatement(stmt) => Some(stmt),
            _ => None,
        };
        assert!(stmt.is_some());
        assert!(stmt.clone().unwrap().value.is_some());

        let expression = match stmt.unwrap().value.unwrap() {
            Expression::CallExpression(call_expression) => Some(call_expression),
            _ => None,
        };
        assert!(expression.is_some());

        match test_identifier(
            Box::from(expression.clone().unwrap().function),
            String::from("add"),
        ) {
            Err(e) => panic!("function identifier failure: {}", e),
            _ => {}
        };

        assert_eq!(3, expression.clone().unwrap().arguments.len());

        let arguments = expression.clone().unwrap().arguments;
        match test_literal_expression(
            Box::from(arguments[0].clone()),
            ExpressionExpectation::Integer(1),
        ) {
            Err(e) => panic!("unexpected first argument: {}", e),
            _ => {}
        }

        match test_infix_expression(
            Box::from(arguments[1].clone()),
            String::from("*"),
            ExpressionExpectation::Integer(2),
            ExpressionExpectation::Integer(3),
        ) {
            Err(e) => panic!("unexpected second argument: {}", e),
            _ => {}
        }

        match test_infix_expression(
            Box::from(arguments[2].clone()),
            String::from("+"),
            ExpressionExpectation::Integer(4),
            ExpressionExpectation::Integer(5),
        ) {
            Err(e) => panic!("unexpected third argument: {}", e),
            _ => {}
        }
    }

    #[derive(Debug, Clone)]
    pub enum ExpressionExpectation {
        Integer(i64),
        Identifier(String),
        Bool(bool),
    }

    fn test_infix_expression(
        expression: Box<Expression>,
        operator: String,
        left: ExpressionExpectation,
        right: ExpressionExpectation,
    ) -> Result<(), String> {
        match *expression {
            Expression::InfixExpression(infix_expression) => {
                assert_eq!(operator, infix_expression.operator);
                assert!(infix_expression.left.is_some());
                assert!(infix_expression.right.is_some());
                match test_literal_expression(infix_expression.left.unwrap(), left) {
                    Err(e) => {
                        return Err(format!("unexpected left expression {}", e));
                    }
                    _ => {}
                }
                match test_literal_expression(infix_expression.right.unwrap(), right) {
                    Err(e) => {
                        return Err(format!("unexpected right expression {}", e));
                    }
                    _ => {}
                }
                Ok(())
            }
            _ => Err(String::from("expected infix expression")),
        }
    }

    fn test_identifier(expression: Box<Expression>, value: String) -> Result<(), String> {
        match *expression {
            Expression::Identifier(identifier_expression) => {
                if identifier_expression.value != value {
                    return Err(format!(
                        "expected identifier expression value {} but received {}",
                        value, identifier_expression.value
                    ));
                }
                if identifier_expression.token_literal() != format!("{}", value) {
                    return Err(format!(
                        "expected identifier expression literal {} but received {}",
                        value, identifier_expression.value
                    ));
                }
                Ok(())
            }
            _ => Err(String::from("expected identifier expressions")),
        }
    }

    fn test_literal_expression(
        expression: Box<Expression>,
        expectation: ExpressionExpectation,
    ) -> Result<(), String> {
        match expectation {
            ExpressionExpectation::Identifier(value) => test_identifier(expression, value),
            ExpressionExpectation::Integer(value) => test_integer_literal(expression, value),
            ExpressionExpectation::Bool(value) => test_boolean_expression(expression, value),
        }
    }

    fn test_integer_literal(expression: Box<Expression>, value: i64) -> Result<(), String> {
        match *expression {
            Expression::IntegerLiteral(integer_literal) => {
                if integer_literal.value != value {
                    return Err(format!(
                        "expected integer literal value {} but received {}",
                        value, integer_literal.value
                    ));
                }
                if integer_literal.token_literal() != format!("{}", value) {
                    return Err(format!(
                        "expected integer literal token literal {} but received {}",
                        value, integer_literal.value
                    ));
                }
                Ok(())
            }
            _ => Err(String::from("expected integeral literal")),
        }
    }

    fn test_boolean_expression(expression: Box<Expression>, value: bool) -> Result<(), String> {
        match *expression {
            Expression::Boolean(boolean_expression) => {
                if boolean_expression.value != value {
                    return Err(format!(
                        "expected boolean value {} but received {}",
                        value, boolean_expression.value
                    ));
                }
                if boolean_expression.token_literal() != format!("{}", value) {
                    return Err(format!(
                        "expected boolean literal {} but received {}",
                        value, boolean_expression.value
                    ));
                }
                Ok(())
            }
            _ => Err(String::from("expected boolean expression")),
        }
    }
}
