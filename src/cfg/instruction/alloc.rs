use super::*;

pub struct Alloc {
    size: PlaceValue,
}

impl Alloc {
    pub fn from(size: PlaceValue) -> Self {
        Self { size }
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

impl Write for Alloc {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "alloc(")?;
        self.size.write(writer, classes, function)?;
        write!(writer, ")")
    }
}
