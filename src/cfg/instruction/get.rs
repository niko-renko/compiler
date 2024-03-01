use super::*;

pub struct Get {
    ptr: PlaceValue,
    offset: PlaceValue,
}

impl Get {
    pub fn from(ptr: PlaceValue, offset: PlaceValue) -> Self {
        Self { ptr, offset }
    }
}

impl Into<Instruction> for Get {
    fn into(self) -> Instruction {
        Instruction::Get(self)
    }
}

impl Used for Get {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.ptr, self.offset]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.ptr, &mut self.offset]
    }
}

impl InstructionHash for Get {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}
