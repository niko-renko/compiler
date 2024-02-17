use super::*;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    And,
    Xor,
    Eq,
}

pub struct Op {
    left: PlaceValue,
    right: PlaceValue,
    operator: Operator,
}

impl Op {
    pub fn from(left: PlaceValue, right: PlaceValue, operator: Operator) -> Self {
        Op {
            left,
            right,
            operator,
        }
    }
}

impl Into<Instruction> for Op {
    fn into(self) -> Instruction {
        Instruction::Op(self)
    }
}
