use crate::ast::fn_literal::FnLiteral;
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::ast::token_node::TokenNode;
use crate::ast::{
    block_statement::BlockStatement, if_expression::IfExpression,
    operators::get_token_type_operator_precedence, trace::Tracer,
};
use crate::ast::{boolean_expression::BooleanExpression, identifier::Identifier};
use crate::ast::{
    expression::Expression, expression_statement::ExpressionStatement,
    infix_expression::InfixExpression, integer_literal::IntegerLiteral,
    let_statement::LetStatement, operators::Operator, prefix_expression::PrefixExpression,
    return_statement::ReturnStatement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    l: &'a mut Lexer<'a>,
    pub errors: Vec<String>,
    pub current_token: Option<Token>,
    pub peek_token: Option<Token>,
    pub tracer: Tracer,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            l: l,
            errors: vec![],
            current_token: None,
            peek_token: None,
            tracer: Tracer::new(true),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn parse_prefix(&mut self, token_type: TokenType) -> Option<Expression> {
        let s = format!("parse_prefix, {}", token_type);
        let untrace = self.tracer.trace(s.as_str());

        match token_type {
            TokenType::IDENT => {
                let identifier = self.parse_identifier();
                untrace(&mut self.tracer);
                Some(identifier)
            }
            TokenType::BANG | TokenType::MINUS => {
                let prefix_expression = self.parse_prefix_expression();
                untrace(&mut self.tracer);
                Some(prefix_expression)
            }
            TokenType::IF => {
                untrace(&mut self.tracer);
                self.parse_if_expression()
            }
            TokenType::LPAREN => {
                untrace(&mut self.tracer);
                self.parse_grouped_expression()
            }
            TokenType::INT => {
                let integer_expression = self.parse_integer();
                untrace(&mut self.tracer);
                Some(integer_expression)
            }
            TokenType::FUNCTION => {
                let fn_expression = self.parse_fn_literal();
                untrace(&mut self.tracer);
                fn_expression
            }
            TokenType::TRUE | TokenType::FALSE => {
                let boolean_expression = self.parse_boolean();
                untrace(&mut self.tracer);
                Some(boolean_expression)
            }
            _ => {
                untrace(&mut self.tracer);
                None
            }
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

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expression = self.parse_expression(Operator::LOWEST);
        if expression.is_none() {
            return None;
        }
        if !self.peek_token_is(TokenType::RPAREN) {
            return None;
        }
        self.next_token();
        Some(expression.unwrap())
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

    fn is_infix_token(&mut self, token_type: TokenType) -> bool {
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

    fn expect_current(&mut self, tt: TokenType) -> bool {
        if self.current_token.is_none() {
            self.errors
                .push(format!("expected current token {}, found none", tt));
            return false;
        }
        let current_token_type = self.current_token.clone().unwrap().token_type;
        if current_token_type != tt {
            self.errors.push(format!(
                "expected current token {}, found {}",
                tt, current_token_type
            ));
            return false;
        }
        return true;
    }

    fn parse_fn_parameters(&mut self) -> Result<Vec<Identifier>, String> {
        let mut parameters: Vec<Identifier> = vec![];

        if !self.current_token_is(TokenType::LPAREN) {
            return Err(format!(
                "expected LPAREN, found {}",
                self.current_token.clone().unwrap().token_type
            ));
        }

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Ok(parameters);
        }
        self.next_token();

        let identifier = Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal,
        };
        parameters.push(identifier.clone());

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();

            let identifier = Identifier {
                token: self.current_token.clone().unwrap(),
                value: self.current_token.clone().unwrap().literal,
            };
            parameters.push(identifier);
        }
        self.next_token();

        if !self.current_token_is(TokenType::RPAREN) {
            return Err(format!(
                "expected RPAREN, found {}",
                self.current_token.clone().unwrap().token_type
            ));
        }

        Ok(parameters)
    }

    fn parse_fn_literal(&mut self) -> Option<Expression> {
        let s = format!(
            "parse_fn_literal {}",
            self.current_token.clone().unwrap().token_type
        );
        let untrace = self.tracer.trace(s.as_str());

        if !self.current_token_is(TokenType::FUNCTION) {
            panic!(
                "expected FUNCTION, found {}",
                self.current_token.clone().unwrap().token_type
            );
        }

        let fn_token = self.current_token.clone().unwrap();
        let peek_token = self.peek_token.clone().unwrap();
        if !self.peek_token_is(TokenType::LPAREN) {
            panic!(
                "expected LPAREN, found {}",
                self.peek_token.clone().unwrap().token_type
            );
        }
        self.next_token();

        let parameters_result = self.parse_fn_parameters();
        match parameters_result {
            Err(err) => panic!(err),
            _ => {}
        }
        let parameters = parameters_result.unwrap();

        if !self.expect_peek(TokenType::LBRACE) {
            self.next_token();
            untrace(&mut self.tracer);
            return None;
        }

        let body = self.parse_block_statement();

        let expression = FnLiteral {
            token: fn_token,
            parameters: parameters,
            body: body,
        };

        untrace(&mut self.tracer);
        Some(Expression::FnLiteral(expression))
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let s = format!("parse_if_expression");
        let untrace = self.tracer.trace(s.as_str());

        let if_token = self.current_token.clone().unwrap();

        if !self.expect_peek(TokenType::LPAREN) {
            self.next_token();
            untrace(&mut self.tracer);
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(Operator::LOWEST);
        if condition.is_none() {
            self.next_token();
            untrace(&mut self.tracer);
            return None;
        }
        if !self.expect_peek(TokenType::RPAREN) {
            self.next_token();
            untrace(&mut self.tracer);
            return None;
        }
        if !self.expect_peek(TokenType::LBRACE) {
            self.next_token();
            untrace(&mut self.tracer);
            return None;
        }
        let consequence = self.parse_block_statement();
        if consequence.is_none() {
            self.next_token();
            untrace(&mut self.tracer);
            panic!("no consequence provided");
        }

        let mut alternative = None;
        if self.current_token_is(TokenType::ELSE) {
            self.next_token();
            self.assert_current_token_type(TokenType::LBRACE);

            let alternative_block = self.parse_block_statement();
            if alternative_block.is_some() {
                alternative = Some(Box::new(alternative_block.unwrap()));
            }
        }

        let expression = IfExpression {
            token: if_token,
            condition: Box::new(condition.unwrap()),
            consequence: Box::new(consequence.unwrap()),
            alternative: alternative,
        };

        untrace(&mut self.tracer);
        Some(Expression::IfExpression(expression))
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let s = format!("parse_block_expression");
        let untrace = self.tracer.trace(s.as_str());

        if self.current_token.is_none() {
            untrace(&mut self.tracer);
            return None;
        }
        let mut statements: Vec<Statement> = vec![];
        let token = self.current_token.clone();
        self.next_token();
        while !self.current_token_is(TokenType::RBRACE) {
            let statement = self.parse_statement();
            if statement.is_some() {
                statements.push(statement.unwrap());
            }
        }
        self.next_token();

        let block_statement = BlockStatement {
            token: token.unwrap(),
            statements: statements,
        };
        untrace(&mut self.tracer);
        Some(block_statement)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> InfixExpression {
        let s = format!(
            "parse_infix_expression {}",
            self.current_token.clone().unwrap().token_type
        );
        let untrace = self.tracer.trace(s.as_str());
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
        untrace(&mut self.tracer);
        infix_expression
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.l.next_token());
    }

    pub fn peek_token_is(&self, tt: TokenType) -> bool {
        if self.peek_token.is_none() {
            return false;
        }
        tt == self.peek_token.clone().unwrap().token_type
    }

    pub fn expect_peek(&mut self, tt: TokenType) -> bool {
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

    pub fn peek_precedence(&self) -> Operator {
        get_token_type_operator_precedence(self.peek_token.clone().unwrap().token_type)
    }

    pub fn current_token_is(&self, tt: TokenType) -> bool {
        if self.current_token.is_none() {
            return false;
        }
        tt == self.current_token.clone().unwrap().token_type
    }

    pub fn assert_current_token_type(&self, tt: TokenType) {
        if self.current_token.is_none() {
            panic!("expected current token {}, but none found", tt);
        }
        if !self.current_token_is(tt) {
            panic!(
                "expected current token {}, but found {}",
                tt,
                self.current_token.clone().unwrap().token_type
            );
        }
    }

    pub fn current_precedence(&self) -> Operator {
        get_token_type_operator_precedence(self.current_token.clone().unwrap().token_type)
    }

    pub fn parse_expression(&mut self, operator: Operator) -> Option<Expression> {
        let s = format!("parse_expression, {}", operator);
        let untrace = self.tracer.trace(s.as_str());

        if self.current_token.is_none() {
            untrace(&mut self.tracer);
            return None;
        }
        let prefix = self.parse_prefix(self.current_token.clone().unwrap().token_type);
        if prefix.is_none() {
            self.errors.push(format!(
                "no prefix parse function for {}",
                self.current_token.clone().unwrap().token_type
            ));
            untrace(&mut self.tracer);
            return None;
        }
        let mut left = prefix.unwrap();
        while !self.peek_token_is(TokenType::SEMICOLON) && operator < self.peek_precedence() {
            let is_infix_token = self.is_infix_token(self.peek_token.clone().unwrap().token_type);
            if is_infix_token {
                self.next_token();
                left = Expression::InfixExpression(self.parse_infix_expression(left));
            } else if self.peek_token_is(TokenType::LPAREN) {
                left = self.parse_call_expression(left);
            } else {
                untrace(&mut self.tracer);
                return Some(left);
            }
        }
        untrace(&mut self.tracer);
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
        if !self.current_token_is(TokenType::RBRACE) {
            if !self.current_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }
            self.next_token();
        }
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
pub mod tests {
    use crate::ast::expression::Expression;
    use crate::ast::token_node::TokenNode;
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    use super::Parser;
    use super::Program;
    use super::Statement;

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
    fn if_expression() {
        let tests = vec![(
            "if (x < y) { x }",
            (
                "<",
                ExpressionExpectation::Identifier(String::from("x")),
                ExpressionExpectation::Identifier(String::from("y")),
            ),
            "x",
        )];
        for (input, (operator, left, right), return_identifier) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(0, p.errors.len());
            assert_eq!(1, program.statements.len());
            let statement = program.statements[0].clone();
            let mut expression: Option<Expression> = None;
            match statement {
                Statement::ExpressionStatement(st) => {
                    assert_eq!(true, st.value.is_some());
                    expression = st.value;
                }
                _ => {
                    assert!(false, "expected expression statement");
                }
            }

            assert!(expression.is_some());
            let maybe_if_expression = match expression.unwrap() {
                Expression::IfExpression(exp) => Some(exp),
                _ => None,
            };

            assert!(maybe_if_expression.is_some(), "expected if expression");
            let if_expression = maybe_if_expression.unwrap();

            let err = match test_infix_expression(
                if_expression.condition,
                String::from(*operator),
                (*left).clone(),
                (*right).clone(),
            ) {
                Ok(()) => None,
                Err(e) => Some(e),
            };
            assert!(err.is_none(), err.unwrap());

            assert_eq!(1, if_expression.consequence.statements.len());
            assert!(if_expression.alternative.is_none());

            let consequence_statement = match if_expression.consequence.statements[0].clone() {
                Statement::ExpressionStatement(expression_statement) => Some(expression_statement),
                _ => None,
            };
            assert!(consequence_statement.is_some());
            assert!(consequence_statement.clone().unwrap().value.is_some());

            let consequence_identifier = consequence_statement.clone().unwrap().value;
            assert!(consequence_identifier.is_some());
            match test_identifier(
                Box::from(consequence_identifier.unwrap()),
                String::from(*return_identifier),
            ) {
                Ok(()) => {}
                Err(e) => assert!(false, e),
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
        let tests: Vec<(&str, &str, ExpressionExpectation)> = vec![
            ("!5", "!", ExpressionExpectation::Integer(5)),
            ("-5", "-", ExpressionExpectation::Integer(5)),
            ("!false", "!", ExpressionExpectation::Bool(false)),
            ("!true", "!", ExpressionExpectation::Bool(true)),
        ];
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
                        Expression::PrefixExpression(prefix_expression) => {
                            assert_eq!(String::from(*operator), prefix_expression.operator);
                            assert!(prefix_expression.right.is_some());
                            let expression = prefix_expression.right.unwrap();
                            match test_literal_expression(expression, (*value).clone()) {
                                Err(e) => {
                                    assert!(false, "{}", e);
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
            (
                "true == true",
                ExpressionExpectation::Bool(true),
                "==",
                ExpressionExpectation::Bool(true),
            ),
            (
                "true != false",
                ExpressionExpectation::Bool(true),
                "!=",
                ExpressionExpectation::Bool(false),
            ),
            (
                "false == false",
                ExpressionExpectation::Bool(false),
                "==",
                ExpressionExpectation::Bool(false),
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
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("add(b * c)", "add((b * c))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
        ];
        for (input, expected_output) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(0, p.errors.len(), "{}", p.errors.join(", "));
            assert_eq!(*expected_output, program.string().as_str());
        }
    }

    #[test]
    fn fn_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let mut l = Lexer::new(input);
        let mut p = Parser::new(&mut l);
        let program = p.parse_program();
        assert_eq!(0, p.errors.len(), "{}", p.errors.join(", "));

        assert_eq!(1, program.statements.len());

        let expression_statement = match program.statements[0].clone() {
            Statement::ExpressionStatement(stmt) => Some(stmt),
            _ => None,
        };
        assert!(expression_statement.is_some());

        let expression = expression_statement.clone().unwrap().value;
        assert!(expression.is_some());

        let fn_literal = match expression.unwrap() {
            Expression::FnLiteral(fn_literal) => Some(fn_literal),
            _ => None,
        };
        assert!(fn_literal.is_some());

        assert_eq!(2, fn_literal.clone().unwrap().parameters.len());

        let parameters = fn_literal.clone().unwrap().parameters;

        let mut err = match test_literal_expression(
            Box::new(Expression::Identifier(parameters[0].clone())),
            ExpressionExpectation::Identifier(String::from("x")),
        ) {
            Ok(()) => None,
            Err(e) => Some(e),
        };
        assert!(err.is_none());

        err = match test_literal_expression(
            Box::new(Expression::Identifier(parameters[1].clone())),
            ExpressionExpectation::Identifier(String::from("y")),
        ) {
            Ok(()) => None,
            Err(e) => Some(e),
        };
        assert!(err.is_none());

        assert!(fn_literal.clone().unwrap().body.is_some());
        assert_eq!(
            1,
            fn_literal.clone().unwrap().body.unwrap().statements.len()
        );

        let body_statement = match fn_literal.unwrap().body.unwrap().statements[0].clone() {
            Statement::ExpressionStatement(exp_stmnt) => Some(exp_stmnt),
            _ => None,
        };
        assert!(body_statement.is_some());

        let body_expression = body_statement.unwrap().value;
        assert!(body_expression.is_some());

        err = match test_infix_expression(
            Box::from(body_expression.clone().unwrap()),
            String::from("+"),
            ExpressionExpectation::Identifier(String::from("x")),
            ExpressionExpectation::Identifier(String::from("y")),
        ) {
            Ok(()) => None,
            Err(e) => Some(e),
        };
        assert!(err.is_none());
    }

    #[test]
    fn parameter_parsing() {
        let tests = vec![
            ("fn() {};", vec![]),
            ("fn(x) {};", vec!["x"]),
            ("fn(x, y, z) {};", vec!["x", "y", "z"]),
        ];
        for (input, expected_parameters) in tests.iter() {
            let mut l = Lexer::new(*input);
            let mut p = Parser::new(&mut l);
            let program = p.parse_program();
            assert_eq!(0, p.errors.len(), "{}", p.errors.join("; "));
            assert_eq!(1, program.statements.len());

            let statement = match program.statements[0].clone() {
                Statement::ExpressionStatement(stmt) => Some(stmt),
                _ => None,
            };
            assert!(statement.is_some(), "expected expression statement");
            assert!(statement.clone().unwrap().value.is_some());

            let fn_literal = match statement.clone().unwrap().value.unwrap() {
                Expression::FnLiteral(fn_literal) => Some(fn_literal),
                _ => None,
            };

            assert!(fn_literal.is_some(), "expected Expression::FnLiteral");
            assert_eq!(
                (*expected_parameters).len(),
                fn_literal.clone().unwrap().parameters.len()
            );

            for (i, param) in (*expected_parameters).iter().enumerate() {
                let identifier =
                    Expression::Identifier(fn_literal.clone().unwrap().parameters[i].clone());
                match test_identifier(Box::new(identifier), String::from(*param)) {
                    Ok(()) => {}
                    Err(e) => panic!("identifier did not match {}", e),
                }
            }
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
