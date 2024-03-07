use super::*;

impl Build for ast::Print {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let place_value = self
            .get_expression()
            .update(cfg, classes, types, function)?;
        cfg.add_placed(Place::None, cfg::Print::from(place_value.into()).into());
        Ok(place_value)
    }
}
