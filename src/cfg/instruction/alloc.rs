use super::*;

pub struct Alloc {
    size: Value,
}

impl Alloc {
    pub fn from(size: Value) -> Self {
        Self { size }
    }
}

impl Into<Instruction> for Alloc {
    fn into(self) -> Instruction {
        Instruction::Alloc(self)
    }
}

impl PlacesRead for Alloc {
    fn places_read(&self) -> Vec<Place> {
        vec![]
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        vec![]
    }
}

impl InstructionHash for Alloc {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, usize>) {
        Self::random_hash(state);
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
