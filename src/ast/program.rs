use super::statement::Statement;
use super::token_node::TokenNode;
use crate::token::TokenType;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl TokenNode for Program {
    fn token_type(&self) -> TokenType {
        if self.statements.len() > 0 {
            self.statements[0].token_type()
        } else {
            TokenType::EOF
        }
    }
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }

    fn string(&self) -> String {
        let mut s = String::from("");
        for statement in self.statements.iter() {
            s.push_str(statement.string().as_str())
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;
    use crate::{
        ast::{
            expression::Expression, identifier::Identifier, let_statement::LetStatement,
            program::Program, statement::Statement, token_node::TokenNode,
        },
        token::Token,
    };

    #[test]
    fn string() {
        let token = Token {
            token_type: TokenType::LET,
            literal: String::from("let"),
        };
        let my_var = String::from("myVar");
        let identifier_token = Token {
            token_type: TokenType::IDENT,
            literal: my_var.clone(),
        };
        let identifier = Identifier {
            token: identifier_token,
            value: my_var.clone(),
        };

        let another_var = String::from("anotherVar");
        let value_token = Token {
            token_type: TokenType::IDENT,
            literal: another_var.clone(),
        };
        let value_identifier = Expression::Identifier(Identifier {
            token: value_token,
            value: another_var.clone(),
        });
        let statement = LetStatement {
            token: token,
            name: identifier,
            value: Some(value_identifier),
        };
        let program = Program {
            statements: vec![Statement::LetStatement(statement)],
        };
        assert_eq!(program.string(), String::from("LET myVar = anotherVar;"));
    }
}
