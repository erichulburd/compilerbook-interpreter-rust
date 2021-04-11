use super::{bool::Bool, integer::Integer, null::Null};

pub enum Object {
    Integer(Integer),
    Bool(Bool),
    Null(Null),
}
