use crate::ast_extract::{Classes, Function};

mod bb;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod value;

use bb::BB;
pub use control_transfer::*;
pub use instruction::*;
pub use label::Label;
pub use place::*;
pub use place_value::PlaceValue;
pub use value::Value;

pub struct CFG {
    current: Label,
    blocks: Vec<BB>,
    edges: Vec<(Label, Label)>,
    next_temp: usize,
}

impl CFG {
    pub fn new() -> Self {
        CFG {
            current: Label::from(0),
            blocks: vec![BB::new()],
            edges: Vec::new(),
            next_temp: 0,
        }
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T, classes: &Classes, function: &Function) {
    }
}

impl CFG {
    fn current(&mut self) -> &mut BB {
        &mut self.blocks[self.current.get_id()]
    }

    fn set_current(&mut self, label: Label) {
        self.current = label;
    }

    pub fn new_block(&mut self) -> Label {
        self.blocks.push(BB::new());
        Label::from(self.blocks.len() - 1)
    }

    pub fn add(&mut self, instruction: Instruction) -> Place {
        let temp = Temp::from(self.next_temp);
        self.next_temp += 1;
        self.current().add((temp.into(), instruction));
        temp.into()
    }

    pub fn add_placed(&mut self, place: Place, instruction: Instruction) {
        self.current().add((place, instruction));
    }

    pub fn end(&mut self, control_transfer: ControlTransfer) {
        match &control_transfer {
            ControlTransfer::Branch(branch) => {
                self.edges.push((self.current, branch.get_true()));
                self.edges.push((self.current, branch.get_false()));
            }
            ControlTransfer::Jump(jump) => {
                self.edges.push((self.current, jump.get_label()));
            }
            _ => {}
        }

        self.current().end(control_transfer);
    }
}

impl CFG {
    pub fn fail_if(&mut self, place: Place, condition: bool, fail_reason: FailReason) {
        let fail_block = self.new_block();
        let success_block = self.new_block();

        let true_branch = if condition { fail_block } else { success_block };
        let false_branch = if condition { success_block } else { fail_block };

        let branch = Branch::from(place, true_branch, false_branch);
        self.end(branch.into());

        self.set_current(fail_block);
        self.end(Fail::from(fail_reason).into());

        self.set_current(success_block);
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
