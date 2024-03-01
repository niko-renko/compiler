use super::*;

impl Build for ast::Call {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object = self.get_object().update(cfg, classes, function)?;
        cfg.fail_if_int(object);

        let mut args = vec![];
        for arg in self.get_args() {
            args.push(arg.update(cfg, classes, function)?.into());
        }

        let method_id = if let Some(id) = classes.get_method_id(self.get_method()) {
            id
        } else {
            return Err(format!("Method not found"));
        };

        let vtable = cfg.add(Get::from(object.into(), Value::from_raw(0).into()).into());
        let method = cfg.add(Get::from(vtable.into(), Value::from_raw(method_id).into()).into());

        cfg.fail_if(method, false, FailReason::NoSuchMethod);
        Ok(cfg.add(cfg::Call::from(method.into(), object.into(), args).into()))
    }
}
