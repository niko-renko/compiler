use super::*;

#[derive(Debug)]
pub struct If {
    condition: Expression,
    true_body: Body,
    false_body: Body,
}

impl Parse for If {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "if", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, true_body) = Body::parse(next, true)?;
        let next = Self::consume_string(next, "else", true)?;
        let (next, false_body) = Body::parse(next, true)?;
        Ok((
            next,
            If {
                condition,
                true_body,
                false_body,
            },
        ))
    }
}

impl Update for If {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let true_block = cfg.new_block();
        let false_block = cfg.new_block();

        let condition = self.condition.update(cfg, classes, function)?;
        cfg.fail_if_ptr(condition);
        let condition = cfg.to_raw(condition);
        cfg.end(Branch::from(condition, true_block, false_block).into());

        cfg.set_current(true_block);
        cfg.end(Jump::from(new_current).into());
        self.true_body.update(cfg, classes, function)?;

        cfg.set_current(false_block);
        cfg.end(Jump::from(new_current).into());
        self.false_body.update(cfg, classes, function)?;

        cfg.set_current(new_current.clone());
        Ok(Place::None)
    }
}
