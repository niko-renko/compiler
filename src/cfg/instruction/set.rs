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

impl Write for Set {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        write!(writer, "setelt(")?;
        self.ptr.write(writer, classes, function)?;
        write!(writer, ", ")?;
        self.offset.write(writer, classes, function)?;
        write!(writer, ", ")?;
        self.value.write(writer, classes, function)?;
        write!(writer, ")")
    }
}
