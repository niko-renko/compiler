use super::*;

#[derive(Clone, Copy)]
pub struct Jump(Label);

impl Jump {
    pub fn from(label: Label) -> Self {
        Jump(label)
    }

    pub fn get_label(&self) -> Label {
        self.0
    }
}

impl Into<ControlTransfer> for Jump {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Jump(self)
    }
}

impl Used for Jump {
    fn used(&self) -> Vec<PlaceValue> {
        vec![]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![]
    }
}
