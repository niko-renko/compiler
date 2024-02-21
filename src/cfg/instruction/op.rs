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

impl Used for Op {
    fn used(&self) -> Vec<PlaceValue> {
        vec![self.left, self.right]
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        vec![&mut self.left, &mut self.right]
    }
}

impl InstructionHash for Op {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, constants: &mut HashMap<Place, Value>) -> Option<Value> {
        let mut left = 0;

        if let PlaceValue::Value(value) = self.left {
            left = value.get_value();
        }

        if let PlaceValue::Place(place) = self.left {
            if let Some(value) = constants.get(&place) {
                left = value.get_value();
            } else {
                return None;
            }
        }

        let mut right = 0;

        if let PlaceValue::Value(value) = self.right {
            right = value.get_value();
        }

        if let PlaceValue::Place(place) = self.right {
            if let Some(value) = constants.get(&place) {
                right = value.get_value();
            } else {
                return None;
            }
        }

        let result = match self.operator {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
            Operator::Or => left | right,
            Operator::And => left & right,
            Operator::Xor => left ^ right,
            Operator::Eq => (left == right) as usize,
        };

        Some(Value::from_raw(result))
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
