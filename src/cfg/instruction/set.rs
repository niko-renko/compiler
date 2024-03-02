use super::*;

pub struct Set {
    ptr: PlaceValue,
    offset: PlaceValue,
    value: PlaceValue,
}

impl Set {
    pub fn from(ptr: PlaceValue, offset: PlaceValue, value: PlaceValue) -> Self {
        Self { ptr, offset, value }
    }

    pub fn get_ptr(&self) -> &PlaceValue {
        &self.ptr
    }

    pub fn get_offset(&self) -> &PlaceValue {
        &self.offset
    }

    pub fn get_value(&self) -> &PlaceValue {
        &self.value
    }
}

impl Into<Instruction> for Set {
    fn into(self) -> Instruction {
        Instruction::Set(self)
    }
}

impl Used for Set {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.ptr, self.offset, self.value]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.ptr, &mut self.offset, &mut self.value]
    }
}

impl InstructionHash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}
