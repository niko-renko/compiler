use super::*;

pub struct Set {
    ptr: Place,
    offset: PlaceValue,
    value: PlaceValue,
}

impl Set {
    pub fn from(ptr: Place, offset: PlaceValue, value: PlaceValue) -> Self {
        Self { ptr, offset, value }
    }
}

impl Into<Instruction> for Set {
    fn into(self) -> Instruction {
        Instruction::Set(self)
    }
}

impl PlacesRead for Set {
    fn places_read(&self) -> Vec<Place> {
        let mut places = vec![self.ptr];
        match self.offset {
            PlaceValue::Place(place) => places.push(place),
            _ => (),
        }
        match self.value {
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
        match &mut self.value {
            PlaceValue::Place(place) => places.push(place),
            _ => (),
        }
        places
    }
}

impl InstructionHash for Set {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &mut HashMap<Place, Value>) -> Option<Value> {
        None
    }
}

impl Write for Set {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
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
