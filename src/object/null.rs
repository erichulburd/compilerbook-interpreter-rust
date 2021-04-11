use super::{object_trait::ObjectTrait, truthiness_trait::Truthiness};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Null {}

impl ObjectTrait for Null {
    fn string(&self) -> String {
        String::from("null")
    }
}

impl Truthiness for Null {
    fn is_truthy(&self) -> bool {
        false
    }
}

pub static NULL: Null = Null {};
