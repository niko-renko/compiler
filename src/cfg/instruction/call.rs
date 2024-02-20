use super::*;

pub struct Call {
    code: Place,
    object: Place,
    args: Vec<PlaceValue>,
}

impl Call {
    pub fn from(code: Place, object: Place, args: Vec<PlaceValue>) -> Self {
        Self { code, object, args }
    }
}

impl Into<Instruction> for Call {
    fn into(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl PlacesRead for Call {
    fn places_read(&self) -> Vec<Place> {
        let mut places = vec![self.code, self.object];
        for arg in &self.args {
            match arg {
                PlaceValue::Place(place) => places.push(*place),
                _ => (),
            }
        }
        places
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        let mut places = vec![&mut self.code, &mut self.object];
        for arg in &mut self.args {
            match arg {
                PlaceValue::Place(place) => places.push(place),
                _ => (),
            }
        }
        places
    }
}

impl InstructionHash for Call {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, Value>) {
        Self::random_hash(state);
    }
}

impl Write for Call {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "call(")?;
        self.code.write(writer, classes, function)?;
        write!(writer, ", ")?;
        self.object.write(writer, classes, function)?;
        for arg in &self.args {
            write!(writer, ", ")?;
            arg.write(writer, classes, function)?;
        }
        write!(writer, ")")
    }
}
