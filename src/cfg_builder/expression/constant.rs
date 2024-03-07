use super::*;

impl Build for Constant {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        _: &Classes,
        _: &Types,
        _: &FunctionContext,
    ) -> Result<Place, String> {
        let constant = Alias::from(Value::from(self.get_value()).into());
        Ok(cfg.add(constant.into()))
    }
}
