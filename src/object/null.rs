use super::object_trait::ObjectTrait;

pub struct Null {}

impl ObjectTrait for Null {
    fn string(&self) -> String {
        String::from("null")
    }
}
