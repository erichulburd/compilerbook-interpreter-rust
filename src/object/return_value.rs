use super::{object::Object, object_trait::ObjectTrait, truthiness_trait::Truthiness};

#[derive(Clone, Debug, Eq)]
pub struct ReturnValue {
    pub value: Object,
}

impl PartialEq for ReturnValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl ObjectTrait for ReturnValue {
    fn string(&self) -> String {
        String::from("RETURN_VALUE")
    }
}

impl Truthiness for ReturnValue {
    fn is_truthy(&self) -> bool {
        self.value.is_truthy()
    }
}
