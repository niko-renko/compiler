use super::*;

impl Build for ast::Return {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let value = self.get_expression().update(cfg, classes, function)?;
        cfg.end(cfg::Return::from(value.into()).into());
        Ok(Place::None)
    }
}
