use super::*;

#[derive(Clone, Copy)]
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

impl PlacesRead for Return {
    fn places_read(&self) -> Vec<Place> {
        self.0.places_read()
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
