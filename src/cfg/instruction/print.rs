use super::*;

pub struct Print {
    value: PlaceValue,
}

impl Print {
    pub fn from(value: PlaceValue) -> Self {
        Print { value }
    }

    pub fn get_value(&self) -> &PlaceValue {
        &self.value
    }
}

impl Into<Instruction> for Print {
    fn into(self) -> Instruction {
        Instruction::Print(self)
    }
}

impl Used for Print {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.value]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.value]
    }
}

impl InstructionHash for Print {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}
