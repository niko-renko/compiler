use super::*;

pub struct BB {
    instructions: Vec<(Place, Instruction)>,
    end: ControlTransfer,
}

impl BB {
    pub fn new() -> Self {
        BB {
            instructions: Vec::new(),
            end: Return::new(Value::from_raw(0).into()).into(),
        }
    }

    pub fn add(&mut self, instruction: (Place, Instruction)) {
        self.instructions.push(instruction);
    }

    pub fn end(&mut self, control_transfer: ControlTransfer) {
        self.end = control_transfer;
    }
}

impl Write for BB {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        for (place, instruction) in &self.instructions {
            if let Place::None = place {
            } else {
                place.write(writer, classes, function)?;
                write!(writer, " = ")?;
            }
            instruction.write(writer, classes, function)?;
            writeln!(writer)?;
        }
        self.end.write(writer, classes, function)?;
        writeln!(writer)
    }
}
