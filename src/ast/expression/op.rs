use super::*;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Op {
    left: Box<Expression>,
    right: Box<Expression>,
    operator: Operator,
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

impl Update for Op {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        function: &Function,
        classes: &Classes,
    ) -> Result<Place, String> {
        let left = self.left.update(cfg, function, classes)?;
        cfg.fail_if_ptr(left);

        let right = self.right.update(cfg, function, classes)?;
        cfg.fail_if_ptr(right);

        match self.operator {
            Operator::Add => {
                let left = cfg.add_const(left, -1);
                Ok(cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Add).into()))
            }
            Operator::Sub => {
                let untagged =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Sub).into());

                Ok(cfg.add_const(untagged, 1))
            }
            Operator::Mul => {
                let left = cfg.to_raw(left);
                let right = cfg.to_raw(right);

                let raw =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Mul).into());

                Ok(cfg.to_int(raw))
            }
            Operator::Div => {
                let is_zero = cfg.add(
                    cfg::Op::from(
                        right.clone().into(),
                        Value::from(0).into(),
                        cfg::Operator::Eq,
                    )
                    .into(),
                );

                cfg.fail_if(is_zero, true, FailReason::NotANumber);

                let left = cfg.add_const(left, -1);
                let right = cfg.add_const(right, -1);

                let raw =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Div).into());

                Ok(cfg.to_int(raw))
            }
        }
    }
}
