use std::borrow::Borrow;

use crate::lexer::Lexer;

use super::{identifier, let_statement::{LetStatement}, return_statement::ReturnStatement};
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
        None
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
          assert_eq!(TokenType::LET, let_statement.token_literal());
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

    for (i, statement) in program.statements.iter().enumerate() {
      match statement {
        Statement::ReturnStatement(st) => {
          assert_eq!(TokenType::RETURN, st.token_literal());
        },
        _ => {
          assert!(false, "all statements should be let statements");
        }
      }

    }
  }
}
