use token::{Token, TokenType};

use crate::token;

pub struct Lexer<'a> {
    input: &'a str,
    position: i32,
    read_position: i32,
    ch: Option<char>,
}

impl Lexer<'_> {
    pub fn new(input: &'_ str) -> Lexer {
        let mut l = Lexer{
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn read_tokens(&mut self) -> Vec<Token> {
        let mut token = self.next_token();
        let mut tokens: Vec<Token> = vec![];
        while token.token_type != TokenType::EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens.push(token);
        tokens
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.unwrap_or('0').is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let multi_char_token = self.read_multi_char_tokens();
        if multi_char_token.is_some() {
            return multi_char_token.unwrap();
        }

        let token_type = token::lookup_char(self.ch);
        if token_type.is_some() {
            let literal = String::from(self.ch.unwrap());
            self.read_char();
            return token::Token{
                token_type: token_type.unwrap(),
                literal: literal,
            };
        }

        if self.ch.unwrap_or('0').is_alphabetic() {
            let literal = self.read_identifier();
            let token_type = token::lookup_keyword(literal.clone());
            return token::Token{
                token_type: token_type,
                literal: literal,
            };
        }
        if self.ch.unwrap_or('a').is_numeric() {
            return token::Token{
                token_type: token::TokenType::INT,
                literal: self.read_number(),
            };
        }
        let s = match self.ch {
            None => String::from(""),
            _ => String::from(self.ch.unwrap()),
        };
        self.read_char();
        token::Token{
            token_type: token::TokenType::EOF,
            literal: s,
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut s = String::new();
        while self.ch.unwrap_or_else(|| '0').is_alphabetic() {
            s.push(self.ch.unwrap());
            self.read_char();
        }
        return s;
    }

    fn read_number(&mut self) -> String {
        let mut s = String::new();
        while self.ch.unwrap_or('a').is_numeric() {
            s.push(self.ch.unwrap());
            self.read_char();
        }
        return s;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() as i32 {
            self.ch = None;
        } else {
            let ch = self.input.chars().nth(self.read_position as usize);
            self.ch = ch;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_multi_char_tokens(&mut self) -> Option<token::Token> {
        match self.ch.unwrap_or('0') {
            '=' => {
                if self.peek_char().unwrap_or('0') == '=' {
                    self.read_char();
                    self.read_char();
                    return Some(Token{
                        token_type: TokenType::EQ,
                        literal: String::from("=="),
                    })
                }
                None
            },
            '!' => {
                if self.peek_char().unwrap_or('0') == '=' {
                    self.read_char();
                    self.read_char();
                    return Some(Token{
                        token_type: TokenType::NotEq,
                        literal: String::from("!="),
                    })
                }
                None
            },
            _ => None,
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position as usize >= self.input.len() {
            return None
        }
        self.input.chars().nth(self.read_position as usize)
    }
}

#[cfg(test)]
mod tests {
    use token::TokenType;

    use crate::token;

    use super::{Lexer};

    #[test]
    fn next_token_basic() {
        let input: &str = "=+(){},;";
        let tests = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RBRACE, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for (k, v) in tests {
            let t = l.next_token();
            assert_eq!(t.token_type, k);
            assert_eq!(t.literal, v);
        }
    }

    #[test]
    fn next_token_program() {
        let input: &str = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let tests = vec![
            (TokenType::LET, "let"),
            (TokenType::IDENT, "five"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::LET, "let"),
            (TokenType::IDENT, "ten"),
            (TokenType::ASSIGN, "="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::LET, "let"),
            (TokenType::IDENT, "add"),
            (TokenType::ASSIGN, "="),
            (TokenType::FUNCTION, "fn"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "x"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "y"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::IDENT, "x"),
            (TokenType::PLUS, "+"),
            (TokenType::IDENT, "y"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::LET, "let"),
            (TokenType::IDENT, "result"),
            (TokenType::ASSIGN, "="),
            (TokenType::IDENT, "add"),
            (TokenType::LPAREN, "("),
            (TokenType::IDENT, "five"),
            (TokenType::COMMA, ","),
            (TokenType::IDENT, "ten"),
            (TokenType::RPAREN, ")"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::BANG, "!"),
            (TokenType::MINUS, "-"),
            (TokenType::SLASH, "/"),
            (TokenType::ASTERISK, "*"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::GT, ">"),
            (TokenType::INT, "5"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::IF, "if"),
            (TokenType::LPAREN, "("),
            (TokenType::INT, "5"),
            (TokenType::LT, "<"),
            (TokenType::INT, "10"),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::TRUE, "true"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),
            (TokenType::ELSE, "else"),
            (TokenType::LBRACE, "{"),
            (TokenType::RETURN, "return"),
            (TokenType::FALSE, "false"),
            (TokenType::SEMICOLON, ";"),
            (TokenType::RBRACE, "}"),

            (TokenType::INT, "10"),
            (TokenType::EQ, "=="),
            (TokenType::INT, "10"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::INT, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::INT, "9"),
            (TokenType::SEMICOLON, ";"),

            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        let mut i = 0;
        for (k, v) in tests {
            let t = l.next_token();
            assert_eq!(t.literal, v, "test {}", i);
            assert_eq!(t.token_type, k, "test {}", i);
            i += 1;
        }
    }
}
