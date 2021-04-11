use interpreter::evaluator::evaluate::evaluate;
use test_integer_object::test_integer_object;
mod test_integer_object;

#[test]
fn evaluate_integer() {
    let tests = vec![("5", 5), ("10", 10)];
    for (input, expected_value) in tests.iter() {
        let obj = match evaluate(*input) {
            Ok(object) => Some(object),
            Err(e) => panic!("{}", e),
        };
        assert!(obj.is_some());
        test_integer_object(obj.unwrap(), *expected_value);
    }
}