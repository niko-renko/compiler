use super::*;

impl Build for ast::Return {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let value = self.get_expression().update(cfg, classes, function)?;
        cfg.fail_if_ptr(value);
        cfg.end(cfg::Return::from(value.into()).into());
        Ok(Place::None)
    }
}
