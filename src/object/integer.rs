use super::object_trait::ObjectTrait;

#[derive(Clone, Copy)]
pub struct Integer {
    pub value: i64,
}

impl ObjectTrait for Integer {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}
