use super::*;

pub struct Return(PlaceValue);

impl Return {
    pub fn new(place_value: PlaceValue) -> Self {
        Return(place_value)
    }
}

impl Into<ControlTransfer> for Return {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Return(self)
    }
}
