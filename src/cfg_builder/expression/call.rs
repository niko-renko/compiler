use super::*;

impl Build for ast::Call {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object = self.get_object().update(cfg, classes, types, function)?;

        let mut args = vec![];
        for arg in self.get_args() {
            args.push(arg.update(cfg, classes, types, function)?.into());
        }

        let method_id = classes.get_method_id(self.get_method()).unwrap();
        let vtable = cfg.add(Get::from(object.into(), Value::from(0).into()).into());
        let method = cfg.add(Get::from(vtable.into(), Value::from(method_id).into()).into());
        Ok(cfg.add(cfg::Call::from(method.into(), object.into(), args).into()))
    }
}
