use super::*;

impl Build for New {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        _: &FunctionContext,
    ) -> Result<Place, String> {
        let class_id = if let Some(id) = classes.get_class_id(&self.get_class_name()) {
            id
        } else {
            return Err(format!("Class not found"));
        };

        let field_count = classes.get_field_count(class_id);
        let object = cfg.add(Alloc::from(Value::from_raw(2 + field_count).into()).into());

        let vtable: Place = Static::vtable(class_id).into();
        cfg.add_placed(
            Place::None,
            Set::from(object.into(), Value::from_raw(0).into(), vtable.into()).into(),
        );

        let field_map: Place = Static::fieldmap(class_id).into();
        cfg.add_placed(
            Place::None,
            Set::from(object.into(), Value::from_raw(1).into(), field_map.into()).into(),
        );

        Ok(object)
    }
}
