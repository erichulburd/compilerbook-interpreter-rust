use super::{bool::Bool, integer::Integer, null::Null};
use super::{object_trait::ObjectTrait, truthiness_trait::Truthiness};

#[derive(Clone, Copy, Debug, Eq)]
pub enum Object {
    Integer(Integer),
    Bool(Bool),
    Null(Null),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(self_int), Object::Integer(other_int)) => {
                return self_int == other_int;
            }
            (Object::Bool(self_bool), Object::Bool(other_bool)) => {
                return self_bool == other_bool;
            }
            (Object::Null(_), Object::Null(_)) => {
                return true;
            }
            _ => {}
        }
        return false;
    }
}

impl Truthiness for Object {
    fn is_truthy(&self) -> bool {
        match self {
            Object::Bool(bool_object) => bool_object.is_truthy(),
            Object::Null(null_object) => null_object.is_truthy(),
            _ => true,
        }
    }
}

impl Object {
    pub fn string(&self) -> String {
        match self {
            Object::Integer(integer) => format!("{}", integer.string()),
            Object::Bool(bool_object) => format!("{}", bool_object.string()),
            Object::Null(null_object) => format!("{}", null_object.string()),
        }
    }
}
