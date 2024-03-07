use super::*;

impl Build for Body {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        for statement in self.get_statements() {
            statement.update(cfg, classes, types, function)?;
        }
        Ok(Place::None)
    }
}
