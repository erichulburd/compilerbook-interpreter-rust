use crate::{
    ast::{expression::Expression, node::Node, program::Program, statement::Statement},
    lexer::Lexer,
    object::bool::Bool,
    object::integer::Integer,
    object::{
        bool::{FALSE, TRUE},
        null::{Null, NULL},
        object::Object,
    },
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
            Expression::Boolean(bool_expression) => {
                evaluate_node(Node::BooleanExpression(bool_expression))
            }
            Expression::PrefixExpression(prefix_expression) => {
                evaluate_node(Node::PrefixExpression(prefix_expression))
            }
            _ => panic!("unexpected statement type"),
        },
        Node::Program(program) => evaluate_statements(program.statements),
        Node::IntegerLiteral(integer) => Ok(Object::Integer(Integer {
            value: integer.value,
        })),
        Node::BooleanExpression(bool_expression) => Ok(Object::Bool(Bool {
            value: bool_expression.value,
        })),
        Node::PrefixExpression(prefix_expression) => {
            let right = evaluate_node(
                Node::Expression(*prefix_expression.right.unwrap()));
            Ok(evaluate_prefix_expression(
                prefix_expression.operator,
                right.unwrap()))
        },
        _ => Err(String::from("unexpected node type")),
    }
}

static BANG: &'static str = "!";
static MINUS: &'static str = "-";

fn evaluate_prefix_expression(operator: String, right: Object) -> Object {
    if operator == BANG {
        return evaluate_bang_operator(right);
    }
    if operator == MINUS {
        return evaluate_minus_operator(right);
    }
    Object::Null(NULL)
}

fn evaluate_bang_operator(right: Object) -> Object {
    match right {
        Object::Bool(bool_object) => {
            if bool_object.value == TRUE.value {
                return Object::Bool(FALSE);
            }
            Object::Bool(TRUE)
        }
        Object::Null(_) => Object::Bool(TRUE),
        _ => Object::Bool(FALSE),
    }
}

fn evaluate_minus_operator(right: Object) -> Object {
    match right {
        Object::Integer(integer_object) => {
            Object::Integer(Integer{
                value: -integer_object.value,
            })
        }
        _ => Object::Null(NULL),
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
