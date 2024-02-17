use super::*;

pub struct Jump(Label);

impl Jump {
    pub fn from(label: Label) -> Self {
        Jump(label)
    }

    pub fn get_label(&self) -> Label {
        self.0
    }
}
