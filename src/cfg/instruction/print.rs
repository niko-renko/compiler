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

impl Write for Print {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "print(")?;
        self.value.write(writer, classes, function)?;
        write!(writer, ")")
    }
}
