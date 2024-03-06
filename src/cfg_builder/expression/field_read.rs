use super::*;

impl Build for FieldRead {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object = self.get_object().update(cfg, classes, function)?;

        let class_name = function.get_class_name().unwrap();
        let fields = classes.get_class_field_ids(class_name).unwrap();
        let field_id = classes.get_field_id(self.get_field().get_name()).unwrap();
        let field_index = fields.iter().position(|&x| x == field_id).unwrap();

        let field_value = cfg.add(Get::from(object.into(), Value::from(field_index).into()).into());
        Ok(field_value)
    }
}
