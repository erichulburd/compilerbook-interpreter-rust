use super::object_trait::ObjectTrait;

pub struct Integer {
    pub value: i64,
}

impl ObjectTrait for Integer {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}
