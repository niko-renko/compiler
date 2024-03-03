use super::*;

pub enum AssignmentLeft {
    Local(Local),
    Empty,
}

impl Parse for AssignmentLeft {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let char = string.chars().next().unwrap();

        if char == '_' {
            let next = Self::consume(string, 1)?;
            Ok((next, AssignmentLeft::Empty))
        } else {
            let (next, local) = Local::parse(string, false)?;
            Ok((next, AssignmentLeft::Local(local)))
        }
    }
}

pub struct Assignment {
    left: AssignmentLeft,
    right: Expression,
}

impl Assignment {
    pub fn get_left(&self) -> &AssignmentLeft {
        &self.left
    }

    pub fn get_right(&self) -> &Expression {
        &self.right
    }
}

impl Parse for Assignment {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let (next, left) = AssignmentLeft::parse(string, true)?;
        let next = Self::consume_string(next, "=", true)?;
        let (next, right) = Expression::parse(next, true)?;
        Ok((next, Assignment { left, right }))
    }
}
