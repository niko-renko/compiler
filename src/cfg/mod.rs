use std::collections::HashMap;

use crate::ast_extract::{Classes, Function};

mod bb;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod traits;
mod value;

use bb::BB;
pub use control_transfer::*;
pub use instruction::*;
pub use label::Label;
pub use place::*;
pub use place_value::PlaceValue;
pub use traits::{PlacesRead, Write};
pub use value::Value;

pub struct CFG {
    current: Label,
    blocks: Vec<(Label, BB)>,
    preds: HashMap<Label, Vec<Label>>,
    next_temp: usize,
}

impl CFG {
    pub fn new() -> Self {
        let label = Label::from(0);
        CFG {
            current: label,
            blocks: vec![(label, BB::new())],
            preds: HashMap::new(),
            next_temp: 0,
        }
    }
}

impl CFG {
    pub fn get_preds(&self, label: Label) -> Vec<Label> {
        self.preds.get(&label).cloned().unwrap_or_default()
    }

    pub fn get_blocks(&self) -> &Vec<(Label, BB)> {
        &self.blocks
    }

    pub fn get_block(&mut self, label: Label) -> &mut BB {
        &mut self.blocks[label.get_id()].1
    }

    fn get_current(&mut self) -> &mut BB {
        &mut self.blocks[self.current.get_id()].1
    }
}

impl CFG {
    fn add_pred(&mut self, label: Label, pred: Label) {
        self.preds.entry(label).or_insert_with(Vec::new).push(pred);
    }

    pub fn set_current(&mut self, label: Label) {
        self.current = label;
    }

    pub fn new_block(&mut self) -> Label {
        let label = Label::from(self.blocks.len());
        self.blocks.push((label, BB::new()));
        label
    }
}

impl CFG {
    pub fn add(&mut self, instruction: Instruction) -> Place {
        let temp = Temp::from(self.next_temp);
        self.next_temp += 1;
        self.get_current().add((temp.into(), instruction));
        temp.into()
    }

    pub fn add_placed(&mut self, place: Place, instruction: Instruction) {
        self.get_current().add((place, instruction));
    }

    pub fn end(&mut self, control_transfer: ControlTransfer) {
        match &control_transfer {
            ControlTransfer::Branch(branch) => {
                self.add_pred(branch.get_true(), self.current);
                self.add_pred(branch.get_false(), self.current);
            }
            ControlTransfer::Jump(jump) => {
                self.add_pred(jump.get_label(), self.current);
            }
            _ => {}
        }

        self.get_current().end(control_transfer);
    }
}

impl CFG {
    pub fn fail_if(&mut self, place: Place, condition: bool, fail_reason: FailReason) {
        let fail_block = self.new_block();
        let success_block = self.new_block();

        let true_branch = if condition { fail_block } else { success_block };
        let false_branch = if condition { success_block } else { fail_block };

        let branch = Branch::from(place, true_branch, false_branch);
        let current_end = self.get_current().get_end();
        self.end(branch.into());

        self.set_current(fail_block);
        self.end(Fail::from(fail_reason).into());

        self.set_current(success_block);
        self.end(current_end);
    }

    pub fn fail_if_ptr(&mut self, place: Place) {
        self.force_type(place, true);
    }

    pub fn fail_if_int(&mut self, place: Place) {
        self.force_type(place, false);
    }
}

impl CFG {
    fn force_type(&mut self, place: Place, int: bool) {
        let is_int =
            self.add(Op::from(place.into(), Value::from_raw(1).into(), Operator::And).into());

        if int {
            self.fail_if(is_int, false, FailReason::NotANumber);
        } else {
            self.fail_if(is_int, true, FailReason::NotAPointer);
        };
    }

    pub fn to_int(&mut self, place: Place) -> Place {
        let shifted =
            self.add(Op::from(place.into(), Value::from_raw(4).into(), Operator::Mul).into());
        self.add(Op::from(shifted.into(), Value::from_raw(1).into(), Operator::Add).into())
    }

    pub fn to_raw(&mut self, place: Place) -> Place {
        self.add(Op::from(place.into(), Value::from_raw(4).into(), Operator::Div).into())
    }

    pub fn add_const(&mut self, place: Place, value: isize) -> Place {
        let operator = if value > 0 {
            Operator::Add
        } else {
            Operator::Sub
        };

        let value = value.abs() as usize;
        self.add(Op::from(place.into(), Value::from_raw(value).into(), operator).into())
    }
}

impl Write for CFG {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        for (label, block) in &self.blocks {
            label.write(writer, classes, function)?;
            if label.get_id() == 0 {
                write!(writer, "{}:\n", function.get_params_sig())?;
            } else {
                write!(writer, ":\n")?;
            }
            block.write(writer, classes, function)?;
        }

        Ok(())
    }
}
