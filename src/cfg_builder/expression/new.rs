use super::*;

impl Build for New {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        _: &FunctionContext,
    ) -> Result<Place, String> {
        let class_id = classes.get_class_id(&self.get_class_name()).unwrap();
        let field_count = classes.get_class_field_ids(self.get_class_name()).len();
        let object = cfg.add(Alloc::from(Value::from(2 + field_count).into()).into());

        let vtable: Place = Static::vtable_from(class_id).into();
        cfg.add_placed(
            Place::None,
            Set::from(object.into(), Value::from(0).into(), vtable.into()).into(),
        );

        let field_map: Place = Static::fieldmap_from(class_id).into();
        cfg.add_placed(
            Place::None,
            Set::from(object.into(), Value::from(1).into(), field_map.into()).into(),
        );

        Ok(object)
    }
}
