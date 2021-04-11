use super::{object_trait::ObjectTrait, truthiness_trait::Truthiness};

#[derive(Clone, Copy, Debug, Eq)]
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

impl Truthiness for Integer {
    fn is_truthy(&self) -> bool {
        self.value != 0
    }
}
