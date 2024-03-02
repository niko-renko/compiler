use super::*;

pub struct Alloc {
    size: PlaceValue,
}

impl Alloc {
    pub fn from(size: PlaceValue) -> Self {
        Self { size }
    }

    pub fn get_size(&self) -> &PlaceValue {
        &self.size
    }
}

impl Into<Instruction> for Alloc {
    fn into(self) -> Instruction {
        Instruction::Alloc(self)
    }
}

impl Used for Alloc {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.size]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.size]
    }
}

impl InstructionHash for Alloc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}
