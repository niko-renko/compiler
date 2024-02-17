use super::*;

pub enum FailReason {
    NotAPointer,
    NotANumber,
    NoSuchField,
    NoSuchMethod,
}

pub struct Fail(FailReason);

impl Fail {
    pub fn from(reason: FailReason) -> Self {
        Fail(reason)
    }
}

impl Into<ControlTransfer> for Fail {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Fail(self)
    }
}
