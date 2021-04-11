use super::object_trait::ObjectTrait;
use super::{bool::Bool, integer::Integer, null::Null};

pub enum Object {
    Integer(Integer),
    Bool(Bool),
    Null(Null),
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
