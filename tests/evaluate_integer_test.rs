use interpreter::evaluator::evaluate::evaluate;
mod shared;

#[test]
fn evaluate_integer() {
    let tests = vec![
        ("5", 5),
        ("10", 10),
        ("-5", -5),
        ("-10", -10),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];
    for (input, expected_value) in tests.iter() {
        let obj = match evaluate(*input) {
            Ok(object) => Some(object),
            Err(e) => panic!("{}", e),
        };
        assert!(obj.is_some());
        shared::test_integer_object(obj.unwrap(), *expected_value);
    }
}
