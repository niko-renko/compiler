use super::*;

pub struct Print {
    value: PlaceValue,
}

impl Print {
    pub fn from(value: PlaceValue) -> Self {
        Print { value }
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

    fn get_constant(&self, _: &mut HashMap<Place, Value>) -> Option<Value> {
        None
    }
}

impl Write for Print {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "print(")?;
        self.value.write(writer, classes, function)?;
        write!(writer, ")")
    }
}
