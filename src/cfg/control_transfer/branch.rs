use super::*;

pub struct Branch {
    condition: Place,
    t: Label,
    f: Label,
}

impl Branch {
    pub fn from(condition: Place, t: Label, f: Label) -> Self {
        Branch { condition, t, f }
    }

    pub fn get_true(&self) -> Label {
        self.t
    }

    pub fn get_false(&self) -> Label {
        self.f
    }
}

impl Into<ControlTransfer> for Branch {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Branch(self)
    }
}
