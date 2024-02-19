use super::*;

pub struct BB {
    instructions: Vec<(Place, Instruction)>,
    end: ControlTransfer,
    next_phi: usize,
}

impl BB {
    pub fn new() -> Self {
        BB {
            instructions: Vec::new(),
            end: Return::from(Value::from_raw(0).into()).into(),
            next_phi: 0,
        }
    }

    pub fn has_phi(&self, local: usize, label: Label) -> bool {
        let phis = &self.instructions[..self.next_phi];

        for (place, instruction) in phis {
            if let Place::Named(named) = place {
                if named.get_id() != local {
                    continue;
                }

                if let Instruction::Phi(phi) = instruction {
                    return phi.has_label(label);
                }
            }
        }

        false
    }

    pub fn add_phi(&mut self, local: usize, label: Label) {
        let phis = &mut self.instructions[..self.next_phi];

        for (place, instruction) in phis {
            if let Place::Named(named) = place {
                if named.get_id() != local {
                    continue;
                }

                if let Instruction::Phi(phi) = instruction {
                    phi.add_entry(*named, label);
                    return;
                }
            }
        }

        let named = Named::from(local);
        let mut phi = Phi::new();
        phi.add_entry(named, label);
        self.instructions
            .insert(self.next_phi, (named.into(), phi.into()));
        self.next_phi += 1;
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

    pub fn get_instructions_mut(&mut self) -> &mut Vec<(Place, Instruction)> {
        &mut self.instructions
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
