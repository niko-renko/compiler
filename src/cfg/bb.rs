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
