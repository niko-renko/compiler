use super::*;

impl Build for Body {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        for statement in self.get_statements() {
            statement.update(cfg, classes, function)?;
        }
        Ok(Place::None)
    }
}
