use super::*;

#[derive(Clone, Copy)]
pub struct Return(PlaceValue);

impl Return {
    pub fn from(place_value: PlaceValue) -> Self {
        Return(place_value)
    }
}

impl Into<ControlTransfer> for Return {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Return(self)
    }
}

impl Used for Return {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.0]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.0]
    }
}

impl Write for Return {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "ret ")?;
        self.0.write(writer, classes, function)
    }
}
