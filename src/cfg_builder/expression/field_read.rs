use super::*;

impl Build for FieldRead {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object = self.get_object().update(cfg, classes, function)?;
        cfg.fail_if_int(object);

        let field_id = if let Some(id) = classes.get_field_id(self.get_field()) {
            id
        } else {
            return Err(format!("Field not found"));
        };

        let field_map = cfg.add(Get::from(object.into(), Value::from_raw(1).into()).into());
        let field_offset =
            cfg.add(Get::from(field_map.into(), Value::from_raw(field_id).into()).into());
        cfg.fail_if(field_offset.clone(), false, FailReason::NoSuchField);

        let field_value = cfg.add(Get::from(object.into(), field_offset.into()).into());
        Ok(field_value)
    }
}
