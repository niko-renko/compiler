use super::*;

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl Parse for Operator {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let char = string.chars().next();
        let operator = match char {
            Some('+') => Operator::Add,
            Some('-') => Operator::Sub,
            Some('*') => Operator::Mul,
            Some('/') => Operator::Div,
            _ => return Err(String::from("Expected arithmetic operator")),
        };
        let next = Self::consume(string, 1)?;
        Ok((next, operator))
    }
}

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
