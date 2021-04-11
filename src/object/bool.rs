use super::object_trait::ObjectTrait;

#[derive(Clone, Copy, Eq)]
pub struct Bool {
    pub value: bool,
}

impl ObjectTrait for Bool {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

impl PartialEq for Bool {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub static TRUE: Bool = Bool { value: true };
pub static FALSE: Bool = Bool { value: false };
