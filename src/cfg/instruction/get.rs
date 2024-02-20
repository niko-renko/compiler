use super::*;

pub struct Get {
    ptr: Place,
    offset: PlaceValue,
}

impl Get {
    pub fn from(ptr: Place, offset: PlaceValue) -> Self {
        Self { ptr, offset }
    }
}

impl Into<Instruction> for Get {
    fn into(self) -> Instruction {
        Instruction::Get(self)
    }
}

impl PlacesRead for Get {
    fn places_read(&self) -> Vec<Place> {
        let mut places = vec![self.ptr];
        match self.offset {
            PlaceValue::Place(place) => places.push(place),
            _ => (),
        }
        places
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        let mut places = vec![&mut self.ptr];
        match &mut self.offset {
            PlaceValue::Place(place) => places.push(place),
            _ => (),
        }
        places
    }
}

impl InstructionHash for Get {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, Value>) {
        Self::random_hash(state);
    }
}

impl Write for Get {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "getelt(")?;
        self.ptr.write(writer, classes, function)?;
        write!(writer, ", ")?;
        self.offset.write(writer, classes, function)?;
        write!(writer, ")")
    }
}
