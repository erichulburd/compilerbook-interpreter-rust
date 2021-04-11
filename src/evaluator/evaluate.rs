use crate::{
    ast::{expression::Expression, node::Node, program::Program, statement::Statement},
    lexer::Lexer,
    object::integer::Integer,
    object::{null::Null, object::Object},
    parser::parser::Parser,
};

pub fn evaluate(input: &str) -> Result<Object, String> {
    let mut l = Lexer::new(input);
    let mut p = Parser::new(&mut l);
    let program: Program = p.parse_program();
    assert_eq!(1, program.statements.len());
    evaluate_node(Node::Program(program))
}

fn evaluate_node(node: Node) -> Result<Object, String> {
    match node {
        Node::Statement(stmt) => match stmt {
            Statement::ExpressionStatement(expression_statement) => {
                evaluate_node(Node::Expression(expression_statement.value.unwrap()))
            }
            _ => panic!("unexpected statement type"),
        },
        Node::Expression(expression) => match expression {
            Expression::IntegerLiteral(integer) => evaluate_node(Node::IntegerLiteral(integer)),
            _ => panic!("unexpected statement type"),
        },
        Node::Program(program) => evaluate_statements(program.statements),
        Node::IntegerLiteral(integer) => Ok(Object::Integer(Integer {
            value: integer.value,
        })),
        _ => Err(String::from("unexpected node type")),
    }
}

fn evaluate_statements(statements: Vec<Statement>) -> Result<Object, String> {
    let mut result = Object::Null(Null {});
    for statement in statements.iter() {
        match evaluate_node(Node::Statement((*statement).clone())) {
            Ok(object) => {
                result = object;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Ok(result);
}
