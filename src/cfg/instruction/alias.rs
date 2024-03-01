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
        if let PlaceValue::Place(place) = &self.0 {
            place.hash(state);
        } else {
            Self::random_hash(state);
        }
    }

    fn get_constant(&self, vn: &HashMap<u64, PlaceValue>) -> Option<Value> {
        match &self.0 {
            PlaceValue::Place(place) => {
                if let Some(PlaceValue::Value(value)) = vn.get(&place.hash_one()) {
                    Some(*value)
                } else {
                    None
                }
            }
            PlaceValue::Value(value) => Some(*value),
        }
    }
}

impl Write for Alias {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        self.0.write(writer, classes, function)
    }
}
