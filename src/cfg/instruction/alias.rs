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

impl Used for Alias {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.0]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.0]
    }
}

impl InstructionHash for Alias {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }

    fn get_constant(&self, constants: &mut HashMap<Place, Value>) -> Option<Value> {
        if let PlaceValue::Value(value) = &self.0 {
            return Some(*value);
        }

        if let PlaceValue::Place(place) = &self.0 {
            return constants.get(place).map(|v| *v);
        }

        None
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
