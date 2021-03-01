#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
}

impl Default for TokenType {
    fn default() -> Self { TokenType::EOF }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Token<'a> {
    pub literal: &'a str,
    pub token_type: TokenType,
}


pub struct Parser<'a> {
  current_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {

  pub fn subroutine(&mut self) -> Option<Token<'a>> {
    Some(self.current_token.unwrap())
  }

  pub fn do_stuff(&mut self) -> Vec<Token> {
    let mut statements: Vec<Token> = vec![];
    let s = self.subroutine();
    if s.is_some() {
      statements.push(s.unwrap());
      let other = self.do_stuff();


    }
    statements
  }
}
