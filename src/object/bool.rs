use super::object_trait::ObjectTrait;

pub struct Bool {
    pub value: bool,
}

impl ObjectTrait for Bool {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}
