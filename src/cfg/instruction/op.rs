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
    pub fn to_string(&self) -> String {
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

    pub fn get_left(&self) -> &PlaceValue {
        &self.left
    }

    pub fn get_operator(&self) -> &Operator {
        &self.operator
    }

    pub fn get_right(&self) -> &PlaceValue {
        &self.right
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

        hashes.sort();
        hashes.hash(state);
        self.operator.to_string().hash(state);
    }

    fn get_constant(&self, vn: &HashMap<u64, PlaceValue>) -> Option<Value> {
        if matches!(self.left, PlaceValue::Place(_)) && matches!(self.right, PlaceValue::Place(_)) {
            let left = match self.left {
                PlaceValue::Place(place) => place,
                _ => unreachable!(),
            };

            let right = match self.right {
                PlaceValue::Place(place) => place,
                _ => unreachable!(),
            };

            if left == right && matches!(self.operator, Operator::Sub | Operator::Xor) {
                return Some(Value::from_raw(0));
            }

            if left == right && matches!(self.operator, Operator::Div) {
                return Some(Value::from_raw(1));
            }
        }

        let left = match self.left {
            PlaceValue::Value(value) => Some(value.into()),
            PlaceValue::Place(place) => vn.get(&place.hash_one()).map(|x| *x),
        }?;

        let left = match left {
            PlaceValue::Value(value) => value.get_value(),
            _ => return None,
        };

        let right = match self.right {
            PlaceValue::Value(value) => Some(value.into()),
            PlaceValue::Place(place) => vn.get(&place.hash_one()).map(|x| *x),
        }?;

        let right = match right {
            PlaceValue::Value(value) => value.get_value(),
            _ => return None,
        };

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
