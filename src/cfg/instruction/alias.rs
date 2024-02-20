use super::*;

pub struct Alias(PlaceValue);

impl Alias {
    pub fn from(place_value: PlaceValue) -> Self {
        Alias(place_value)
    }

    pub fn get_place_value(&self) -> &PlaceValue {
        &self.0
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

impl InstructionHash for Alias {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, Value>) {
        if let PlaceValue::Place(place) = &self.0 {
            if let Some(value) = constants.get(place) {
                let place_value: PlaceValue = (*value).into();
                place_value.hash(state);
                return;
            }
        }
        self.0.hash(state);
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
