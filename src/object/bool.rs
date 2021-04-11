use super::object_trait::ObjectTrait;

#[derive(Clone, Copy)]
pub struct Bool {
    pub value: bool,
}

impl ObjectTrait for Bool {
    fn string(&self) -> String {
        format!("{}", self.value)
    }
}

pub static TRUE: Bool = Bool { value: true };
pub static FALSE: Bool = Bool { value: false };
