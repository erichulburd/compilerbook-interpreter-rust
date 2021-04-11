use interpreter::object::{integer::Integer, object::Object};

pub fn test_bool_object(obj: Object, expected: bool) {
    let value = match obj {
        Object::Bool(bool_object) => Some(bool_object.value),
        _ => None,
    };
    assert!(value.is_some());
    assert_eq!(expected, value.unwrap());
}

pub fn test_integer_object(obj: Object, expected: i64) {
    let value = match obj {
        Object::Integer(int) => Some(int.value),
        _ => None,
    };
    assert!(value.is_some());
    assert_eq!(expected, value.unwrap());
}

pub fn test_null_object(obj: Object) -> bool {
    match obj {
        Object::Null(int) => {
            return true;
        }
        _ => {}
    };
    return false;
}
