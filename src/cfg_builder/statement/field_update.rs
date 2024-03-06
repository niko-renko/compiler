use super::*;

impl Build for FieldUpdate {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let object = self.get_object().update(cfg, classes, function)?;
        let value = self.get_value().update(cfg, classes, function)?;

        // BUG -- READ FROM SUPPLIED CLASS
        let class_name = function.get_class_name().unwrap();
        let fields = classes.get_class_field_ids(class_name).unwrap();
        let field_id = classes.get_field_id(self.get_field().get_name()).unwrap();
        let field_index = fields.iter().position(|&x| x == field_id).unwrap();

        let set = Set::from(
            object.into(),
            Value::from(1 + field_index).into(),
            value.into(),
        );

        cfg.add_placed(Place::None, set.into());
        Ok(value)
    }
}
