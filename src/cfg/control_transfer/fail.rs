use super::*;

#[derive(Clone, Copy)]
pub enum FailReason {
    NotAPointer,
    NotANumber,
    NoSuchField,
    NoSuchMethod,
}

#[derive(Clone, Copy)]
pub struct Fail(FailReason);

impl Fail {
    pub fn from(reason: FailReason) -> Self {
        Fail(reason)
    }

    pub fn get_reason(&self) -> &FailReason {
        &self.0
    }
}

impl Into<ControlTransfer> for Fail {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Fail(self)
    }
}

impl Used for Fail {
    fn used(&self) -> Vec<PlaceValue> {
        vec![]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![]
    }
}
