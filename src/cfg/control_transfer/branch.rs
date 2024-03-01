use super::*;

#[derive(Clone, Copy)]
pub struct Branch {
    condition: PlaceValue,
    true_label: Label,
    false_label: Label,
}

impl Branch {
    pub fn from(condition: PlaceValue, true_label: Label, false_label: Label) -> Self {
        Branch {
            condition,
            true_label,
            false_label,
        }
    }

    pub fn get_true(&self) -> Label {
        self.true_label
    }

    pub fn get_false(&self) -> Label {
        self.false_label
    }
}

impl Into<ControlTransfer> for Branch {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Branch(self)
    }
}

impl Used for Branch {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.condition]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.condition]
    }
}
