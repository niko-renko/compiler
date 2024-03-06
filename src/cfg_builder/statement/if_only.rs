use super::*;

impl Build for IfOnly {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let true_block = cfg.new_block();

        let condition = self.get_condition().update(cfg, classes, function)?;
        cfg.end(Branch::from(condition.into(), true_block, new_current).into());

        cfg.set_current(true_block);
        cfg.end(Jump::from(new_current).into());
        self.get_body().update(cfg, classes, function)?;

        cfg.set_current(new_current);
        Ok(Place::None)
    }
}
