use super::object_trait::ObjectTrait;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Null {}

impl ObjectTrait for Null {
    fn string(&self) -> String {
        String::from("null")
    }
}

pub static NULL: Null = Null {};
