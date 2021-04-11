use interpreter::evaluator::evaluate::evaluate;
mod shared;

#[test]
fn evaluate_bool() {
    let tests = vec![("true", true), ("false", false)];
    for (input, expected_value) in tests.iter() {
        let obj = match evaluate(*input) {
            Ok(object) => Some(object),
            Err(e) => panic!("{}", e),
        };
        assert!(obj.is_some());
        shared::test_bool_object(obj.unwrap(), *expected_value);
    }
}
