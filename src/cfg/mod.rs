use std::collections::HashMap;

use crate::ast_extract::{Classes, FunctionContext};

mod bb;
mod bfs;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod traits;
mod value;

pub use bb::BB;
pub use bfs::BFSIter;
pub use control_transfer::*;
pub use instruction::*;
pub use label::Label;
pub use place::*;
pub use place_value::PlaceValue;
pub use traits::{InstructionHash, Used};
pub use value::Value;

pub struct CFG {
    current: Label,
    blocks: Vec<BB>,
    edges_in: HashMap<Label, Vec<Label>>,
    edges_out: HashMap<Label, Vec<Label>>,
    next_temp: usize,
}

impl IntoIterator for &CFG {
    type Item = Label;
    type IntoIter = BFSIter;

    fn into_iter(self) -> Self::IntoIter {
        BFSIter::from(self.edges_out.clone(), Label::from(0))
    }
}

impl CFG {
    pub fn new() -> Self {
        let label = Label::from(0);
        CFG {
            current: label,
            blocks: vec![BB::new()],
            edges_in: HashMap::new(),
            edges_out: HashMap::new(),
            next_temp: 0,
        }
    }
}

impl CFG {
    pub fn get_block(&self, label: Label) -> &BB {
        &self.blocks[label.get_id()]
    }

    pub fn get_block_mut(&mut self, label: Label) -> &mut BB {
        &mut self.blocks[label.get_id()]
    }

    pub fn get_edges_in(&self, label: Label) -> Vec<Label> {
        self.edges_in.get(&label).cloned().unwrap_or_default()
    }

    pub fn get_edges_out(&self, label: Label) -> Vec<Label> {
        self.edges_out.get(&label).cloned().unwrap_or_default()
    }

    fn get_current_mut(&mut self) -> &mut BB {
        &mut self.blocks[self.current.get_id()]
    }
}

impl CFG {
    fn add_edge(&mut self, from: Label, to: Label) {
        self.edges_in.entry(to).or_insert_with(Vec::new).push(from);
        self.edges_out.entry(from).or_insert_with(Vec::new).push(to);
    }

    pub fn set_current(&mut self, label: Label) {
        self.current = label;
    }

    pub fn new_block(&mut self) -> Label {
        self.blocks.push(BB::new());
        Label::from(self.blocks.len() - 1)
    }
}

impl CFG {
    pub fn add(&mut self, instruction: Instruction) -> Place {
        let temp = Temp::from(self.next_temp);
        self.next_temp += 1;
        self.get_current_mut().add((temp.into(), instruction));
        temp.into()
    }

    pub fn add_placed(&mut self, place: Place, instruction: Instruction) {
        self.get_current_mut().add((place, instruction));
    }

    pub fn end(&mut self, control_transfer: ControlTransfer) {
        match &control_transfer {
            ControlTransfer::Branch(branch) => {
                self.add_edge(self.current, branch.get_true());
                self.add_edge(self.current, branch.get_false());
            }
            ControlTransfer::Jump(jump) => {
                self.add_edge(self.current, jump.get_label());
            }
            _ => {}
        }

        self.get_current_mut().end(control_transfer);
    }
}

impl CFG {
    pub fn fail_if(&mut self, place: Place, condition: bool, fail_reason: FailReason) {
        let fail_block = self.new_block();
        let success_block = self.new_block();

        let true_branch = if condition { fail_block } else { success_block };
        let false_branch = if condition { success_block } else { fail_block };

        let branch = Branch::from(place.into(), true_branch, false_branch);
        let current_end = self.get_current_mut().get_end();
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
