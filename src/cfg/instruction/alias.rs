use super::*;

pub struct Alias(PlaceValue);

impl Alias {
    pub fn from(place_value: PlaceValue) -> Self {
        Alias(place_value)
    }
}

impl Into<Instruction> for Alias {
    fn into(self) -> Instruction {
        Instruction::Alias(self)
    }
}

impl PlacesRead for Alias {
    fn places_read(&self) -> Vec<Place> {
        self.0.places_read()
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        self.0.places_read_mut()
    }
}

impl Write for Alias {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        self.0.write(writer, classes, function)
    }
}
