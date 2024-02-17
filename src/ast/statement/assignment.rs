use super::*;

#[derive(Debug)]
pub enum AssignmentLeft {
    Local(Local),
    Empty,
}

impl Update for AssignmentLeft {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        match self {
            AssignmentLeft::Local(local) => local.update(cfg, classes, function),
            AssignmentLeft::Empty => Ok(Place::None),
        }
    }
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

#[derive(Debug)]
pub struct Assignment {
    left: AssignmentLeft,
    right: Expression,
}

impl Parse for Assignment {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let (next, left) = AssignmentLeft::parse(string, true)?;
        let next = Self::consume_string(next, "=", true)?;
        let (next, right) = Expression::parse(next, true)?;
        Ok((next, Assignment { left, right }))
    }
}

impl Update for Assignment {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let left = self.left.update(cfg, classes, function)?;
        let right = self.right.update(cfg, classes, function)?;
        cfg.add_placed(left, Alias::from(right.into()).into());
        Ok(left)
    }
}
