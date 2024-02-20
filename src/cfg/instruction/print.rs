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

impl PlacesRead for Print {
    fn places_read(&self) -> Vec<Place> {
        self.value.places_read()
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        self.value.places_read_mut()
    }
}

impl InstructionHash for Print {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, Value>) {
        Self::random_hash(state);
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
