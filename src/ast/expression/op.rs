use super::*;

#[derive(PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Parse for Operator {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let mut chars = string.chars();
        let char = string.chars().next();

        let operator = match char {
            Some('+') => Some(Operator::Add),
            Some('-') => Some(Operator::Sub),
            Some('*') => Some(Operator::Mul),
            Some('/') => Some(Operator::Div),
            Some('=') => {
                if chars.next() == Some('=') {
                    Some(Operator::Eq)
                } else {
                    None
                }
            }
            _ => None,
        };

        let operator = match operator {
            Some(operator) => operator,
            None => return Err(format!("Expected operator, found {}", char.unwrap())),
        };

        let mut length = 1;

        if operator == Operator::Eq {
            length = 2;
        }

        let next = Self::consume(string, length)?;
        Ok((next, operator))
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Op {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Operator,
}

impl Op {
    pub fn get_left(&self) -> &Expression {
        &self.left
    }

    pub fn get_right(&self) -> &Expression {
        &self.right
    }

    pub fn get_operator(&self) -> &Operator {
        &self.operator
    }
}

impl Parse for Op {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "(", false)?;
        let (next, left) = Expression::parse(next, true)?;
        let (next, operator) = Operator::parse(next, true)?;
        let (next, right) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ")", true)?;

        Ok((
            next,
            Op {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            },
        ))
    }
}
