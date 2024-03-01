use super::*;

impl Build for ast::Print {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let place_value = self.get_expression().update(cfg, classes, function)?;
        cfg.fail_if_ptr(place_value);
        let raw = cfg.to_raw(place_value);
        cfg.add_placed(Place::None, cfg::Print::from(raw.into()).into());
        Ok(place_value)
    }
}
