use super::*;

pub struct BB {
    phis: Vec<(Named, Phi)>,
    instructions: Vec<(Place, Instruction)>,
    end: ControlTransfer,
}

impl BB {
    pub fn new() -> Self {
        BB {
            phis: Vec::new(),
            instructions: Vec::new(),
            end: Return::new(Value::from_raw(0).into()).into(),
        }
    }

    pub fn has_phi(&self, local: usize, label: Label) -> bool {
        self.phis
            .iter()
            .find(|(n, phi)| n.get_id() == local && phi.has_label(label))
            .is_some()
    }

    pub fn add_phi(&mut self, local: usize, label: Label) {
        let named = Named::from(local);
        let phi = self.phis.iter_mut().find(|(n, _)| n.get_id() == local);

        if let Some((_, phi)) = phi {
            phi.add_entry(named, label);
        } else {
            self.phis.push((named, Phi::from(vec![(named, label)])));
        }
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
        for (named, phi) in &self.phis {
            named.write(writer, classes, function)?;
            write!(writer, " = ")?;
            phi.write(writer, classes, function)?;
            writeln!(writer)?;
        }

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
