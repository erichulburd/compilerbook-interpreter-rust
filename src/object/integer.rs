use super::object_trait::ObjectTrait;

#[derive(Clone, Copy, Eq)]
pub struct Integer {
    pub value: i64,
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl ObjectTrait for Integer {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}
