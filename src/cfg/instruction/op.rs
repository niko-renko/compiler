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

impl Write for Operator {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        write!(
            writer,
            " {} ",
            match self {
                Operator::Add => "+",
                Operator::Sub => "-",
                Operator::Mul => "*",
                Operator::Div => "/",
                Operator::Or => "|",
                Operator::And => "&",
                Operator::Xor => "^",
                Operator::Eq => "==",
            }
        )
    }
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

impl PlacesRead for Op {
    fn places_read(&self) -> Vec<Place> {
        let mut places = self.left.places_read();
        places.extend(self.right.places_read());
        places
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        let mut places = self.left.places_read_mut();
        places.extend(self.right.places_read_mut());
        places
    }
}

impl Write for Op {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        self.left.write(writer, classes, function)?;
        self.operator.write(writer, classes, function)?;
        self.right.write(writer, classes, function)
    }
}
