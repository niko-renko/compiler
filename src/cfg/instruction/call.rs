use super::*;

pub struct Call {
    code: PlaceValue,
    object: PlaceValue,
    args: Vec<PlaceValue>,
}

impl Call {
    pub fn from(code: PlaceValue, object: PlaceValue, args: Vec<PlaceValue>) -> Self {
        Self { code, object, args }
    }
}

impl Into<Instruction> for Call {
    fn into(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl Used for Call {
    fn used(&self) -> Vec<PlaceValue> {
        let mut used = vec![self.code, self.object];
        used.extend(self.args.clone());
        used
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        let mut used = vec![&mut self.code, &mut self.object];
        used.extend(self.args.iter_mut());
        used
    }
}

impl InstructionHash for Call {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}

impl Write for Call {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &FunctionContext,
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
