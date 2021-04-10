use super::parser::Parser;
use crate::{
    ast::operators::Operator,
    ast::{return_statement::ReturnStatement},
    token::{Token, TokenType},
};

impl<'a> Parser<'a> {

  pub fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
    if self.current_token.is_none() {
        return None;
    }

    let t = self.current_token.clone().unwrap();
    let token = Token {
        token_type: t.token_type,
        literal: t.literal,
    };

    let value = self.parse_expression(Operator::LOWEST);

    let stmt = ReturnStatement {
        token: token,
        value: value,
    };
    if self.peek_token_is(TokenType::SEMICOLON) {
      self.next_token();
    }
    self.next_token();
    Some(stmt)
  }

}

#[cfg(test)]
mod tests {
    use crate::ast::expression::Expression;
    use crate::ast::program::Program;
    use crate::ast::statement::Statement;
    use crate::ast::token_node::TokenNode;
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    use super::Parser;

    fn return_statements() {
      let tests = vec![
        ("return x;", ExpressionExpectation::Identifier(String::from("x"))),
        ("return 10;", ExpressionExpectation::Integer(10)),
        ("return false;", ExpressionExpectation::Bool(false)),
      ];

      for (input, expected_value) in tests.iter() {
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program: Program = p.parse_program();
        assert_eq!(
            1,
            program.statements.len(),
            "unexpected number of statements parsed"
        );
        assert_eq!(0, p.errors.len());

        let statement = &program.statements[0];
        let return_statment = match statement {
            Statement::ReturnStatement(return_statement) => Some(return_statement),
            _ => None,
        };
        assert!(return_statment.is_some());
        assert_eq!(TokenType::RETURN, return_statment.clone().unwrap().token_type());

        let value = return_statment.unwrap().clone().value;
        assert!(value.is_some());

        match test_literal_expression(
          Box::from(value.unwrap()),
          (*expected_value).clone()) {
            Err(e) => panic!("{}", e),
            _ => {}
        };
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
