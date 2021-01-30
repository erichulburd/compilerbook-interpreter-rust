use crate::token;

struct Lexer {
    input: &'static str,
    position: i32,
    read_position: i32,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &'static str) -> Lexer {
        let mut l = Lexer{
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.unwrap_or('0').is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let token = match self.ch {
            Some('=') => {
                token::Token{
                    token_type: token::TokenType::ASSIGN,
                    literal: String::from("="),
                }
            },
            Some(';') => token::Token{
                token_type: token::TokenType::SEMICOLON,
                literal: String::from(";"),
            },
            Some('(') => token::Token{
                token_type: token::TokenType::LPAREN,
                literal: String::from("("),
            },
            Some(')') => token::Token{
                token_type: token::TokenType::RPAREN,
                literal: String::from(")"),
            },
            Some('{') => token::Token{
                token_type: token::TokenType::LBRACE,
                literal: String::from("{"),
            },
            Some('}') => token::Token{
                token_type: token::TokenType::RBRACE,
                literal: String::from("}"),
            },
            Some(',') => token::Token{
                token_type: token::TokenType::COMMA,
                literal: String::from(","),
            },
            Some('+') => token::Token{
                token_type: token::TokenType::PLUS,
                literal: String::from("+"),
            },
            _ => {
                if self.ch.unwrap_or('0').is_alphabetic() {
                    let literal = self.read_identifier();
                    let token_type = token::lookup_keyword(literal.clone());
                    return token::Token{
                        token_type: token_type,
                        literal: literal,
                    };
                } else if self.ch.unwrap_or('a').is_numeric() {
                    return token::Token{
                        token_type: token::TokenType::INT,
                        literal: self.read_number(),
                    };
                } else {
                    let s = match self.ch {
                        None => String::from(""),
                        _ => String::from(self.ch.unwrap()),
                    };
                    token::Token{
                        token_type: token::TokenType::EOF,
                        literal: s,
                    }
                }
            }
        };
        self.read_char();
        token
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
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        let mut i = 0;
        for (k, v) in tests {
            let t = l.next_token();
            assert_eq!(t.token_type, k, "test {}", i);
            assert_eq!(t.literal, v, "test {}", i);
            i += 1;
        }
    }
}
