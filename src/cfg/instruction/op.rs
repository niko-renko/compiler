use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

impl Operator {
    fn get_char(&self) -> String {
        match self {
            Operator::Add => String::from("+"),
            Operator::Sub => String::from("-"),
            Operator::Mul => String::from("*"),
            Operator::Div => String::from("/"),
            Operator::Or => String::from("|"),
            Operator::And => String::from("&"),
            Operator::Xor => String::from("^"),
            Operator::Eq => String::from("=="),
        }
    }
}

impl Write for Operator {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, " {} ", self.get_char())
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
        let mut hashes = vec![];

        for place in self.used() {
            let mut hasher = DefaultHasher::new();
            place.hash(&mut hasher);
            hashes.push(hasher.finish());
        }

        hashes.sort().hash(state);
        self.operator.get_char().hash(state);
        Self::random_hash(state);
    }

    fn get_constant(
        &self,
        constants: &mut HashMap<Place, Value>,
        vn: &HashMap<u64, usize>,
    ) -> Option<Value> {
        if matches!(self.left, PlaceValue::Place(_)) && matches!(self.right, PlaceValue::Place(_)) {
            let left = match self.left {
                PlaceValue::Place(place) => place,
                _ => unreachable!(),
            };

            let mut hasher = DefaultHasher::new();
            left.hash(&mut hasher);
            let left_hash = hasher.finish();
            let left_vn = vn.get(&left_hash);

            let right = match self.right {
                PlaceValue::Place(place) => place,
                _ => unreachable!(),
            };

            let mut hasher = DefaultHasher::new();
            right.hash(&mut hasher);
            let right_hash = hasher.finish();
            let right_vn = vn.get(&right_hash);

            if matches!(self.operator, Operator::Div) && left_vn == right_vn {
                return Some(Value::from_raw(1));
            }
        }

        let left = match self.left {
            PlaceValue::Value(value) => Some(value.get_value()),
            PlaceValue::Place(place) => constants.get(&place).map(|x| x.get_value()),
        }?;

        let right = match self.right {
            PlaceValue::Value(value) => Some(value.get_value()),
            PlaceValue::Place(place) => constants.get(&place).map(|x| x.get_value()),
        }?;

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
