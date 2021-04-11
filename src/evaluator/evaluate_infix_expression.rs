use crate::object::{integer::Integer, null::NULL, object::Object};

pub fn evaluate_infix_expression(
  operator: String,
  left: Object,
  right: Object,
) -> Object {
  match left {
    Object::Integer(left_int) => {
      match right {
        Object::Integer(right_int) => {
          return evaluate_integer_infix_expression(
            left_int,
            right_int,
            operator,
          );
        }
        _ => {}
      }
    }
    _ => {}
  }

  Object::Null(NULL)
}


fn evaluate_integer_infix_expression(
  left: Integer,
  right: Integer,
  operator: String,
) -> Object {
  if operator == "+" {
    return Object::Integer(Integer{
      value: left.value + right.value
    })
  }
  if operator == "-" {
    return Object::Integer(Integer{
      value: left.value - right.value
    })
  }
  if operator == "*" {
    return Object::Integer(Integer{
      value: left.value * right.value
    })
  }
  if operator == "/" {
    return Object::Integer(Integer{
      value: left.value / right.value
    })
  }

  Object::Null(NULL)
}
