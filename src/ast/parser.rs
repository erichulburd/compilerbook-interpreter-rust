use scopeguard::guard;
use std::collections::HashMap;

use super::operators::get_token_type_operator_precedence;
use super::program::Program;
use super::statement::Statement;
use super::{boolean_expression::BooleanExpression, identifier::Identifier};
use super::{
    expression::Expression, expression_statement::ExpressionStatement,
    infix_expression::InfixExpression, integer_literal::IntegerLiteral,
    let_statement::LetStatement, operators::Operator, prefix_expression::PrefixExpression,
    return_statement::ReturnStatement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    errors: Vec<String>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l: l,
            errors: vec![],
            current_token: None,
            peek_token: None,
        };
        p.next_token();
        p.next_token();
        p
    }

    fn parse_prefix(&mut self, token_type: TokenType) -> Option<Expression> {
        defer!(println!("parse prefix"));
        match token_type {
            TokenType::IDENT => {
                let identifier = self.parse_identifier();
                Some(identifier)
            }
            TokenType::BANG | TokenType::MINUS => {
                let prefix_expression = self.parse_prefix_expression();
                Some(prefix_expression)
            }
            TokenType::INT => {
                let integer_expression = self.parse_integer();
                Some(integer_expression)
            }
            TokenType::TRUE | TokenType::FALSE => {
                let boolean_expression = self.parse_boolean();
                Some(boolean_expression)
            }
            _ => None,
        }
    }

    fn parse_identifier(&self) -> Expression {
        let token = self.current_token.clone().unwrap();
        let literal = String::from(token.literal.as_str());

        Expression::Identifier(Identifier {
            token: token,
            value: literal,
        })
    }

    fn parse_integer(&self) -> Expression {
        let token = self.current_token.clone().unwrap();
        let literal = token.literal.parse::<i64>().unwrap();

        Expression::IntegerLiteral(IntegerLiteral {
            token: token,
            value: literal,
        })
    }

    fn parse_boolean(&self) -> Expression {
        let token = self.current_token.clone().unwrap();
        let value = token.token_type == TokenType::TRUE;

        Expression::Boolean(BooleanExpression {
            token: token,
            value: value,
        })
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.current_token.clone().unwrap();
        let literal = token.literal.clone();
        self.next_token();
        let right = self.parse_expression(Operator::PREFIX);
        let mut right_box = None;
        if right.is_some() {
            right_box = Some(Box::new(right.unwrap()));
        }
        Expression::PrefixExpression(PrefixExpression {
            token: token,
            operator: literal,
            right: right_box,
        })
    }

    fn is_infix_token(&mut self, token_type: TokenType, left: Expression) -> bool {
        match token_type {
            TokenType::PLUS
            | TokenType::MINUS
            | TokenType::SLASH
            | TokenType::ASTERISK
            | TokenType::EQ
            | TokenType::NotEq
            | TokenType::GT
            | TokenType::LT => true,
            _ => false,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> InfixExpression {
        let token = self.current_token.clone().unwrap();
        let operator = token.clone().literal;
        let precedence = self.current_precedence();
        self.next_token();

        let right = self.parse_expression(precedence);
        let infix_expression = InfixExpression {
            token: token,
            operator: operator,
            left: Some(Box::new(left)),
            right: if right.is_some() {
                Some(Box::new(right.unwrap()))
            } else {
                None
            },
        };

        infix_expression
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token());
    }

    fn peek_token_is(&self, tt: TokenType) -> bool {
        if self.peek_token.is_none() {
            return false;
        }
        tt == self.peek_token.clone().unwrap().token_type
    }

    fn expect_peek(&mut self, tt: TokenType) -> bool {
        if self.peek_token_is(tt) {
            self.next_token();
            true
        } else if self.peek_token.clone().is_none() {
            self.errors.push(String::from(format!(
                "expected next token to be {}, but none exists",
                tt
            )));
            false
        } else {
            self.errors.push(String::from(format!(
                "expected next token to be {}, got {} instead",
                tt,
                self.peek_token.clone().unwrap().token_type
            )));
            false
        }
    }

    fn peek_precedence(&self) -> Operator {
        get_token_type_operator_precedence(self.peek_token.clone().unwrap().token_type)
    }

    fn current_token_is(&self, tt: TokenType) -> bool {
        if self.current_token.is_none() {
            return false;
        }
        tt == self.current_token.clone().unwrap().token_type
    }

    fn current_precedence(&self) -> Operator {
        get_token_type_operator_precedence(self.current_token.clone().unwrap().token_type)
    }

    fn parse_let_statement(&mut self) -> Option<LetStatement> {
        if self.current_token.is_none() {
            return None;
        }
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let t = self.current_token.clone().unwrap();
        let token = Token {
            token_type: t.token_type,
            literal: t.literal,
        };
        let literal = String::from(token.literal.as_str());
        let identifier = Identifier {
            token: token,
            value: literal,
        };
        let literal2 = self.current_token.clone().unwrap().literal;
        let token2 = Token {
            token_type: t.token_type,
            literal: literal2,
        };

        let stmt = LetStatement {
            token: token2,
            name: identifier,
            value: None, // FIXME
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        self.next_token();
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        self.next_token();
        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<ReturnStatement> {
        if self.current_token.is_none() {
            return None;
        }

        let t = self.current_token.clone().unwrap();
        let token = Token {
            token_type: t.token_type,
            literal: t.literal,
        };

        let stmt = ReturnStatement {
            token: token,
            value: None, // FIXME
        };
        self.next_token();
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        self.next_token();
        Some(stmt)
    }

    fn parse_expression(&mut self, operator: Operator) -> Option<Expression> {
        if self.current_token.is_none() {
            return None;
        }
        let prefix = self.parse_prefix(self.current_token.clone().unwrap().token_type);
        if prefix.is_none() {
            self.errors.push(format!(
                "no prefix parse function for {}",
                self.current_token.clone().unwrap().token_type
            ));
            return None;
        }
        let mut left = prefix.unwrap();
        while !self.peek_token_is(TokenType::SEMICOLON) && operator < self.peek_precedence() {
            let is_infix_token =
                self.is_infix_token(self.peek_token.clone().unwrap().token_type, left.clone());
            if !is_infix_token {
                return Some(left);
            }
            self.next_token();
            left = Expression::InfixExpression(self.parse_infix_expression(left));
        }
        Some(left)
    }

    fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
        if self.current_token.is_none() {
            return None;
        } else if self.current_token_is(TokenType::EOF) {
            return None;
        }

        let t = self.current_token.clone().unwrap();
        let expression = self.parse_expression(Operator::LOWEST);
        let expression_statement = ExpressionStatement {
            token: t,
            value: expression,
        };
        self.next_token();
        if !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        self.next_token();
        Some(expression_statement)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if self.current_token.is_none() {
            return None;
        }
        let token = self.current_token.clone().unwrap();
        match token.token_type {
            TokenType::LET => {
                let let_statement = self.parse_let_statement();
                if let_statement.is_none() {
                    return None;
                }
                Some(Statement::LetStatement(let_statement.unwrap()))
            }
            TokenType::RETURN => {
                let st = self.parse_return_statement();
                if st.is_none() {
                    return None;
                }
                Some(Statement::ReturnStatement(st.unwrap()))
            }
            _ => {
                let st = self.parse_expression_statement();
                if st.is_none() {
                    return None;
                }
                Some(Statement::ExpressionStatement(st.unwrap()))
            }
        }
    }

    pub fn parse_statements(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        loop {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                statements.push(stmt.unwrap());
            } else {
                break;
            }
        }

        statements
    }

    pub fn parse_program(&mut self) -> Program {
        Program {
            statements: self.parse_statements(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::expression::Expression;
    use super::super::token_node::TokenNode;
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    use super::Parser;
    use super::Program;
    use super::Statement;

    #[test]
    fn let_statements() {
        let input = "\
    let x = 5;\
    let y = 10;\
    let foobar = 838383;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program: Program = p.parse_program();
        assert_eq!(
            3,
            program.statements.len(),
            "unexpected number of statements parsed"
        );
        assert_eq!(0, p.errors.len());

        let tests = vec![("x"), ("y"), ("foobar")];
        for (i, id) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            match statement {
                Statement::LetStatement(let_statement) => {
                    assert_eq!(TokenType::LET, let_statement.token_type());
                    assert_eq!(String::from(*id), let_statement.name.value);
                }
                _ => {
                    assert!(false, "all statements should be let statements");
                }
            }
        }
    }

    #[test]
    fn return_statements() {
        let input = "\
    return 5;\
    return 10;\
    return 838383;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program: Program = p.parse_program();
        assert_eq!(
            3,
            program.statements.len(),
            "unexpected number of statements parsed"
        );
        assert_eq!(0, p.errors.len());

        for (_, statement) in program.statements.iter().enumerate() {
            match statement {
                Statement::ReturnStatement(st) => {
                    assert_eq!(TokenType::RETURN, st.token_type());
                }
                _ => {
                    assert!(false, "all statements should be let statements");
                }
            }
        }
    }

    #[test]
    fn identifier_expression() {
        let input = "foobar;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(1, program.statements.len());
        let statement = program.statements[0].clone();
        match statement {
            Statement::ExpressionStatement(st) => {
                assert_eq!(TokenType::IDENT, st.token_type());
                assert_eq!(true, st.value.is_some());
                let expression = st.value.unwrap();
                match test_identifier(Box::from(expression), String::from("foobar")) {
                    Ok(()) => {}
                    Err(e) => assert!(false, "{}", e),
                }
            }
            _ => {
                assert!(false, "expected expression statement");
            }
        }
    }

    #[test]
    fn integer_literal_expression() {
        let input = "5;";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(1, program.statements.len());
        let statement = program.statements[0].clone();
        match statement {
            Statement::ExpressionStatement(st) => {
                assert_eq!(TokenType::INT, st.token_type());
                assert_eq!(true, st.value.is_some());
                let expression = st.value.unwrap();
                match test_integer_literal(Box::from(expression), 5) {
                    Ok(()) => {}
                    Err(e) => assert!(false, "{}", e),
                }
            }
            _ => {
                assert!(false, "expected expression statement");
            }
        }
    }

    #[test]
    fn integer_boolean_expression() {
        let tests = vec![("true;", true), ("false;", false)];
        for (input, value) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(1, program.statements.len());
            let statement = program.statements[0].clone();
            match statement {
                Statement::ExpressionStatement(st) => {
                    assert_eq!(true, st.value.is_some());
                    let expression = st.value.unwrap();
                    match test_boolean_expression(Box::from(expression), *value) {
                        Ok(()) => {}
                        Err(e) => assert!(false, "{}", e),
                    }
                }
                _ => {
                    assert!(false, "expected expression statement");
                }
            }
        }
    }

    #[test]
    fn parse_prefix_expression() {
        let tests: Vec<(&str, &str, i64)> = vec![("!5", "!", 5), ("-5", "-", 5)];
        for (input, operator, value) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(0, p.errors.len());

            assert_eq!(1, program.statements.len());
            let statement = program.statements[0].clone();
            match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    let expression = expression_statement.value.clone().unwrap();
                    match expression {
                        // Expression::Identifier(prefix_expression) => {
                        Expression::PrefixExpression(prefix_expression) => {
                            assert_eq!(String::from(*operator), prefix_expression.operator);
                            assert!(prefix_expression.right.is_some());
                            match test_integer_literal(prefix_expression.right.unwrap(), *value) {
                                Err(e) => {
                                    assert!(false, format!("{}", e));
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            assert!(false, "expected prefix expression");
                        }
                    }
                }
                _ => {
                    assert!(false, "expected expression statement");
                }
            }
        }
    }

    #[test]
    fn parse_infix_expression() {
        let tests: Vec<(&str, ExpressionExpectation, &str, ExpressionExpectation)> = vec![
            (
                "5 + 6;",
                ExpressionExpectation::Integer(5),
                "+",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 - 6;",
                ExpressionExpectation::Integer(5),
                "-",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 * 6;",
                ExpressionExpectation::Integer(5),
                "*",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 / 6;",
                ExpressionExpectation::Integer(5),
                "/",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 > 6;",
                ExpressionExpectation::Integer(5),
                ">",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 < 6;",
                ExpressionExpectation::Integer(5),
                "<",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 == 6;",
                ExpressionExpectation::Integer(5),
                "==",
                ExpressionExpectation::Integer(6),
            ),
            (
                "5 != 6;",
                ExpressionExpectation::Integer(5),
                "!=",
                ExpressionExpectation::Integer(6),
            ),
        ];
        for (input, left_value, operator, right_value) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(1, program.statements.len());
            assert_eq!(0, p.errors.len(), "{}", p.errors.join(", "));

            let statement = program.statements[0].clone();
            match statement {
                Statement::ExpressionStatement(expression_statement) => {
                    let expression = expression_statement.value.clone().unwrap();
                    match test_infix_expression(
                        Box::from(expression),
                        String::from(*operator),
                        (*left_value).clone(),
                        (*right_value).clone(),
                    ) {
                        Ok(()) => {}
                        Err(e) => {
                            assert!(false, "{}", e);
                        }
                    }
                }
                _ => {
                    assert!(false, "expected expression statement");
                }
            }
        }
    }

    #[test]
    fn precedence_parsing() {
        let tests: Vec<(&str, &str)> = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)")
        ];
        for (input, expected_output) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(0, p.errors.len(), "{}", p.errors.join(", "));
            assert_eq!(*expected_output, program.string().as_str());
        }
    }

    #[derive(Debug, Clone)]
    enum ExpressionExpectation {
        Integer(i64),
        Identifier(String),
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

    fn test_literal_expression(
        expression: Box<Expression>,
        expectation: ExpressionExpectation,
    ) -> Result<(), String> {
        match expectation {
            ExpressionExpectation::Identifier(value) => test_identifier(expression, value),
            ExpressionExpectation::Integer(value) => test_integer_literal(expression, value),
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
}
