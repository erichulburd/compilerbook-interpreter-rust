use interpreter::evaluator::evaluate::evaluate;
use interpreter::object::{object::Object};
mod shared;

#[test]
fn return_value() {
    let tests = vec![
      ("return 10;", Object::new_integer(10)),
      ("return 10; 9;", Object::new_integer(10)),
      ("return 2 * 5; 9;", Object::new_integer(10)),
      ("9; return 2 * 5; 9;", Object::new_integer(10)),
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
