use super::*;

impl Build for FieldRead {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object_type = types
            .get_type(function.get_function_name(), self.get_object())
            .unwrap();

        let class_name = if let Type::Object(class_name) = object_type {
            class_name
        } else {
            unreachable!();
        };

        let object = self.get_object().update(cfg, classes, types, function)?;
        let fields = classes.get_class_field_ids(class_name).unwrap();
        let field_id = classes.get_field_id(self.get_field().get_name()).unwrap();
        let field_index = fields.iter().position(|&x| x == field_id).unwrap();

        let field_value =
            cfg.add(Get::from(object.into(), Value::from(1 + field_index).into()).into());

        Ok(field_value)
    }
}
