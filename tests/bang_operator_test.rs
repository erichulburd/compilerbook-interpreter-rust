use interpreter::evaluator::evaluate::evaluate;
mod shared;

#[test]
fn bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];
    for (input, expected_value) in tests.iter() {
        let obj = match evaluate(*input) {
            Ok(object) => Some(object),
            Err(e) => panic!("{}", e),
        };
        assert!(obj.is_some());
        println!("{}", *input);
        shared::test_bool_object(obj.unwrap(), *expected_value);
    }
}
