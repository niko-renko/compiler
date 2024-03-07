use super::*;

impl Build for If {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let new_current = cfg.new_block();
        let true_block = cfg.new_block();
        let false_block = cfg.new_block();

        let condition = self.get_condition().update(cfg, classes, types, function)?;
        cfg.end(Branch::from(condition.into(), true_block, false_block).into());

        cfg.set_current(true_block);
        cfg.end(Jump::from(new_current).into());
        self.get_true_body().update(cfg, classes, types, function)?;

        cfg.set_current(false_block);
        cfg.end(Jump::from(new_current).into());
        self.get_false_body()
            .update(cfg, classes, types, function)?;

        cfg.set_current(new_current);
        Ok(Place::None)
    }
}
