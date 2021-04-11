use interpreter::evaluator::evaluate::evaluate;
use interpreter::object::{bool::Bool, integer::Integer, null::Null, object::Object};
mod shared;

#[test]
fn if_else_expression() {
    let tests = vec![
        ("if (true) { 10 }", Object::Integer(Integer { value: 10 })),
        ("if (false) { 10 }", Object::Null(Null {})),
        ("if (1) { 10 }", Object::Integer(Integer { value: 10 })),
        ("if (1 < 2) { 10 }", Object::Integer(Integer { value: 10 })),
        ("if (1 > 2) { 10 }", Object::Null(Null {})),
        (
            "if (1 > 2) { 10 } else { 20 }",
            Object::Integer(Integer { value: 20 }),
        ),
        (
            "if (1 < 2) { 10 } else { 20 }",
            Object::Integer(Integer { value: 10 }),
        ),
    ];
    for (input, expected_value) in tests.iter() {
        let obj = match evaluate(*input) {
            Ok(object) => Some(object),
            Err(e) => panic!("{}", e),
        };
        assert!(obj.is_some(), "{}", *input);
        assert_eq!(*expected_value, obj.unwrap(), "{}", *input);
    }
}
