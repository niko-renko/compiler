use super::*;

impl Build for Null {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        _: &Classes,
        _: &FunctionContext,
    ) -> Result<Place, String> {
        let constant = Alias::from(Value::from(0).into());
        Ok(cfg.add(constant.into()))
    }
}
