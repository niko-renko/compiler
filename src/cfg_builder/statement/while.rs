use super::*;

impl Build for While {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let condition_block = cfg.new_block();
        let body_block = cfg.new_block();

        cfg.end(Jump::from(condition_block).into());
        cfg.set_current(condition_block);
        let condition = self.get_condition().update(cfg, classes, types, function)?;
        cfg.end(Branch::from(condition.into(), body_block, new_current).into());

        cfg.set_current(body_block);
        self.get_body().update(cfg, classes, types, function)?;
        cfg.end(Jump::from(condition_block).into());

        cfg.set_current(new_current);
        Ok(Place::None)
    }
}
