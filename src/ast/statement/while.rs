use super::*;

#[derive(Debug)]
pub struct While {
    condition: Expression,
    body: Body,
}

impl Parse for While {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "while", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, body) = Body::parse(next, true)?;
        Ok((next, While { condition, body }))
    }
}

impl Update for While {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let condition_block = cfg.new_block();
        let body_block = cfg.new_block();

        cfg.end(Jump::from(condition_block).into());
        cfg.set_current(condition_block);
        let condition = self.condition.update(cfg, classes, function)?;
        cfg.fail_if_ptr(condition);
        let condition = cfg.to_raw(condition);
        cfg.end(Branch::from(condition, body_block, new_current).into());

        cfg.set_current(body_block);
        self.body.update(cfg, classes, function)?;
        cfg.end(Jump::from(condition_block).into());

        cfg.set_current(new_current);
        Ok(Place::None)
    }
}
