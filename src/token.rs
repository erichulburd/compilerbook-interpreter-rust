use std::{borrow::Cow, fmt};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    EQ,
    NotEq,

    IDENT,
    INT,

    LET,
    FUNCTION,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

impl Default for TokenType {
    fn default() -> Self { TokenType::EOF }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

impl fmt::Display for Token {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}: {}", self.token_type, self.literal)
    }
}

pub fn lookup_char(ch: Option<char>) -> Option<TokenType> {
    if ch.is_none() {
        return None;
    }
    match ch.unwrap() {
        '=' => Some(TokenType::ASSIGN),
        ';' => Some(TokenType::SEMICOLON),
        '(' => Some(TokenType::LPAREN),
        ')' => Some(TokenType::RPAREN),
        '{' => Some(TokenType::LBRACE),
        '}' => Some(TokenType::RBRACE),
        ',' => Some(TokenType::COMMA),
        '+' => Some(TokenType::PLUS),
        '-' => Some(TokenType::MINUS),
        '!' => Some(TokenType::BANG),
        '*' => Some(TokenType::ASTERISK),
        '/' => Some(TokenType::SLASH),
        '<' => Some(TokenType::LT),
        '>' => Some(TokenType::GT),
        _ => None,
    }
}

pub fn lookup_keyword(literal: &str) -> TokenType {
    match &literal[..] {
        "fn" => TokenType::FUNCTION,
        "let" => TokenType::LET,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "true" => TokenType::TRUE,
        "false" => TokenType::FALSE,
        _ => TokenType::IDENT,
    }
}


#[cfg(test)]
mod tests {
    use super::{TokenType, lookup_keyword};

    #[test]
    fn get_keyword() {
        assert_eq!(TokenType::LET, lookup_keyword("let"));
        assert_eq!(TokenType::FUNCTION, lookup_keyword("fn"));
        assert_eq!(TokenType::IDENT, lookup_keyword("blah"));
    }

}
