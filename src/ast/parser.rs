use std::{collections::HashMap};

use crate::{lexer::Lexer};

use super::{expression::Expression, expression_statement::ExpressionStatement, integer_literal::IntegerLiteral, let_statement::{LetStatement}, operators::Operator, prefix_expression::PrefixExpression, return_statement::ReturnStatement};
use super::identifier::Identifier;
use super::statement::Statement;
use super::program::Program;
use crate::token::{Token, TokenType};

pub struct Parser<'a> {
  l: &'a mut Lexer<'a>,
  errors: Vec<String>,
  current_token: Option<Token>,
  peek_token: Option<Token>,

}

impl<'a> Parser<'a> {
  pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
    let mut p = Parser{
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
    match token_type {
      TokenType::IDENT => {
        let identifier = self.parse_identifier();
        Some(identifier)
      },
      TokenType::BANG => {
        let identifier = self.parse_prefix_expression();
        Some(identifier)
      },
      TokenType::MINUS => {
        let identifier = self.parse_prefix_expression();
        Some(identifier)
      },
      TokenType::INT => {
        let identifier = self.parse_integer();
        Some(identifier)
      },
      _ => {
        self.errors.push(format!("no prefix parse function for {}", token_type));
        None
      },
    }
  }

  fn parse_identifier(&self) -> Expression {
    let token = self.current_token.clone().unwrap();
    let literal = String::from(token.literal.as_str());

    Expression::Identifier(Identifier{
      token: token,
      value: literal,
    })
  }

  fn parse_integer(&self) -> Expression {
    let token = self.current_token.clone().unwrap();
    let literal = token.literal.parse::<i64>().unwrap();

    Expression::IntegerLiteral(IntegerLiteral{
      token: token,
      value: literal,
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
    Expression::PrefixExpression(PrefixExpression{
      token: token,
      operator: literal,
      right: right_box,
    })
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
    } else if self.peek_token.clone().is_none(){
      self.errors.push(String::from(format!("expected next token to be {}, but none exists", tt)));
      false
    } else {
      self.errors.push(String::from(format!("expected next token to be {}, got {} instead", tt, self.peek_token.clone().unwrap().token_type)));
      false
    }
  }

  fn current_token_is(&self, tt: TokenType) -> bool {
    if self.current_token.is_none() {
      return false;
    }
    tt == self.current_token.clone().unwrap().token_type
  }

  fn parse_let_statement(&mut self) -> Option<LetStatement> {
    if self.current_token.is_none() {
      return None;
    }
    if !self.expect_peek(TokenType::IDENT) {
      return None;
    }

    let t = self.current_token.clone().unwrap();
    let token = Token{
      token_type: t.token_type,
      literal: t.literal,
    };
    let literal = String::from(token.literal.as_str());
    let identifier = Identifier{
      token: token,
      value: literal,
    };
    let literal2 = self.current_token.clone().unwrap().literal;
    let token2 = Token{
      token_type: t.token_type,
      literal: literal2,
    };

    let stmt = LetStatement{
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
    let token = Token{
      token_type: t.token_type,
      literal: t.literal,
    };

    let stmt = ReturnStatement{
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

    let t = self.current_token.clone().unwrap();
    self.parse_prefix(t.token_type)
  }

  fn parse_expression_statement(&mut self) -> Option<ExpressionStatement> {
    if self.current_token.is_none() {
      return None;
    } else if self.current_token_is(TokenType::EOF) {
      return None;
    }

    let t = self.current_token.clone().unwrap();
    let expression = self.parse_expression(Operator::LOWEST);
    let expression_statement = ExpressionStatement{
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
      },
      TokenType::RETURN => {
        let st = self.parse_return_statement();
        if st.is_none() {
          return None;
        }
        Some(Statement::ReturnStatement(st.unwrap()))
      },
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
    Program{
      statements: self.parse_statements(),
    }
  }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::{TokenType};
    use super::super::token_node::TokenNode;
    use super::super::expression::Expression;

    use super::Program;
    use super::Statement;
    use super::Parser;

  #[test]
  fn let_statements() {
    let input = "\
    let x = 5;\
    let y = 10;\
    let foobar = 838383;";
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program: Program = p.parse_program();
    assert_eq!(3, program.statements.len(), "unexpected number of statements parsed");
    assert_eq!(0, p.errors.len());

    let tests = vec![
      ("x"),
      ("y"),
      ("foobar"),
    ];
    for (i, id) in tests.iter().enumerate() {
      let statement = &program.statements[i];
      match statement {
        Statement::LetStatement(let_statement) => {
          assert_eq!(TokenType::LET, let_statement.token_type());
          assert_eq!(String::from(*id), let_statement.name.value);
        },
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
    assert_eq!(3, program.statements.len(), "unexpected number of statements parsed");
    assert_eq!(0, p.errors.len());

    for (_, statement) in program.statements.iter().enumerate() {
      match statement {
        Statement::ReturnStatement(st) => {
          assert_eq!(TokenType::RETURN, st.token_type());
        },
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
        assert_eq!(String::from("foobar"), expression.string());
        match expression {
          Expression::Identifier(identifier) => {
            assert_eq!(String::from("foobar"), identifier.value)
          },
          _ => {
            assert!(false, "expected identifier expression");
          }
        }
      },
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
        assert_eq!(String::from("5"), expression.string());
        match expression {
          Expression::IntegerLiteral(integer_literal) => {
            assert_eq!(5, integer_literal.value)
          },
          _ => {
            assert!(false, "expected integer literal");
          }
        }
      },
      _ => {
        assert!(false, "expected expression statement");
      }
    }
  }

  fn test_integer_literal(expression: Box<Expression>, value: i64) -> Result<bool, String> {
    match *expression {
      Expression::IntegerLiteral(integer_literal) => {
        if integer_literal.value != value {
          return Err(
            format!(
              "expected integer literal value {} but received {}",
              value,
              integer_literal.value));
        }
        if integer_literal.token_literal() != format!("{}", value) {
          return Err(
            format!(
              "expected integer literal token literal {} but received {}",
              value,
              integer_literal.value));
        }
        Ok(true)
      },
      _ => {
        Err(String::from("expected integeral literal"))
      }
    }
  }

  #[test]
  fn parse_prefix_expression() {
    let tests: Vec<(&str, &str, i64)> = vec![
      ("!5", "!", 5),
      ("-5", "-", 5),
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
            // Expression::Identifier(prefix_expression) => {
            Expression::PrefixExpression(prefix_expression) => {
              assert_eq!(String::from(*operator), prefix_expression.operator);
              assert!(prefix_expression.right.is_some());
              match test_integer_literal(prefix_expression.right.unwrap(), *value) {
                Err(e) => {
                  assert!(false, format!("{}", e));
                },
                _ => {},
              }
            },
            _ => {
              assert!(false, "expected prefix expression");
            }
          }
        },
        _ => {
          assert!(false, "expected expression statement");
        }
      }
    }
  }
}
