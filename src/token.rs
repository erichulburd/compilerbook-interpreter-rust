use std::string;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    LET,
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
}

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

pub fn lookup_keyword(literal: String) -> TokenType {
    match &literal[..] {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        _ => TokenType::IDENT,
    }
}


#[cfg(test)]
mod tests {
    use super::{TokenType, lookup_keyword};

    #[test]
    fn get_keyword() {
        assert_eq!(TokenType::LET, lookup_keyword(String::from("let")));
        assert_eq!(TokenType::FUNCTION, lookup_keyword(String::from("fn")));
        assert_eq!(TokenType::IDENT, lookup_keyword(String::from("blah")));
    }

}
