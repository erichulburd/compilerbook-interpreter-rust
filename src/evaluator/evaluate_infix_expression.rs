use crate::object::{bool::Bool, integer::Integer, null::NULL, object::Object};

pub fn evaluate_infix_expression(operator: String, left: Object, right: Object) -> Object {
    match operator.as_str() {
        "==" => {
            return Object::Bool(Bool {
                value: left == right,
            });
        }
        "!=" => {
            return Object::Bool(Bool {
                value: left != right,
            });
        }
        _ => {}
    }
    match left {
        Object::Integer(left_int) => match right {
            Object::Integer(right_int) => {
                return evaluate_integer_infix_expression(left_int, right_int, operator);
            }
            _ => {}
        },
        _ => {}
    }

    Object::Null(NULL)
}

fn evaluate_integer_infix_expression(left: Integer, right: Integer, operator: String) -> Object {
    if operator == "+" {
        return Object::Integer(Integer {
            value: left.value + right.value,
        });
    } else if operator == "-" {
        return Object::Integer(Integer {
            value: left.value - right.value,
        });
    } else if operator == "*" {
        return Object::Integer(Integer {
            value: left.value * right.value,
        });
    } else if operator == "/" {
        return Object::Integer(Integer {
            value: left.value / right.value,
        });
    } else if operator == ">" {
        return Object::Bool(Bool {
            value: left.value > right.value,
        });
    } else if operator == "<" {
        return Object::Bool(Bool {
            value: left.value < right.value,
        });
    }

    Object::Null(NULL)
}
