use super::*;

#[derive(Debug)]
pub struct IfOnly {
    condition: Expression,
    body: Body,
}

impl Parse for IfOnly {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "ifonly", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, body) = Body::parse(next, true)?;

        Ok((next, IfOnly { condition, body }))
    }
}

impl Update for IfOnly {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let true_block = cfg.new_block();

        let condition = self.condition.update(cfg, classes, function)?;
        cfg.fail_if_ptr(condition);
        let condition = cfg.to_raw(condition);
        cfg.end(Branch::from(condition, true_block, new_current).into());

        cfg.set_current(true_block);
        cfg.end(Jump::from(new_current).into());
        self.body.update(cfg, classes, function)?;

        cfg.set_current(new_current);
        Ok(Place::None)
    }
}
