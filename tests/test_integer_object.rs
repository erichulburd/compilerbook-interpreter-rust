use interpreter::object::object::Object;

pub fn test_integer_object(obj: Object, expected: i64) {
    let value = match obj {
        Object::Integer(int) => Some(int.value),
        _ => None,
    };
    assert!(value.is_some());
    assert_eq!(expected, value.unwrap());
}
