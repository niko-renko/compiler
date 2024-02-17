use super::*;

pub struct Alias(PlaceValue);

impl Alias {
    pub fn from(place_value: PlaceValue) -> Self {
        Alias(place_value)
    }
}

impl Into<Instruction> for Alias {
    fn into(self) -> Instruction {
        Instruction::Alias(self)
    }
}
