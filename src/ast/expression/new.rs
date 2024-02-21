use super::*;

#[derive(Debug)]
pub struct New {
    class_name: Name,
}

impl Parse for New {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "@", false)?;
        let (next, class_name) = Name::parse(next, false)?;
        Ok((next, New { class_name }))
    }
}

impl Update for New {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        _: &Function,
    ) -> Result<Place, String> {
        let class_id = if let Some(id) = classes.get_class_id(&self.class_name) {
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
