use super::*;

pub struct BB {
    phis: HashMap<usize, Vec<Label>>,
    instructions: Vec<(Place, Instruction)>,
    end: ControlTransfer,
}

impl BB {
    pub fn new() -> Self {
        BB {
            phis: HashMap::new(),
            instructions: Vec::new(),
            end: Return::new(Value::from_raw(0).into()).into(),
        }
    }

    pub fn has_phi(&self, local: usize, label: Label) -> bool {
        if let Some(labels) = self.phis.get(&local) {
            labels.contains(&label)
        } else {
            false
        }
    }

    pub fn add_phi(&mut self, local: usize, label: Label) {
        self.phis.entry(local).or_insert(Vec::new()).push(label);
    }

    pub fn add(&mut self, instruction: (Place, Instruction)) {
        self.instructions.push(instruction);
    }

    pub fn end(&mut self, control_transfer: ControlTransfer) {
        self.end = control_transfer;
    }

    pub fn get_end(&self) -> ControlTransfer {
        self.end
    }

    pub fn get_instructions(&self) -> &Vec<(Place, Instruction)> {
        &self.instructions
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
        write!(writer, "\n\n")
    }
}
