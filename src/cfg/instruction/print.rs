use super::*;

pub struct Print {
    value: PlaceValue,
}

impl Print {
    pub fn from(value: PlaceValue) -> Self {
        Print { value }
    }
}

impl Into<Instruction> for Print {
    fn into(self) -> Instruction {
        Instruction::Print(self)
    }
}
