use super::*;

#[derive(Debug)]
pub struct FieldUpdate {
    object: Expression,
    field: Local,
    value: Expression,
}

impl Parse for FieldUpdate {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "!", false)?;
        let (next, object) = Expression::parse(next, false)?;
        let next = Self::consume_string(next, ".", false)?;
        let (next, field) = Local::parse(next, false)?;
        let next = Self::consume_string(next, "=", true)?;
        let (next, value) = Expression::parse(next, true)?;

        Ok((
            next,
            FieldUpdate {
                object,
                field,
                value,
            },
        ))
    }
}

impl Update for FieldUpdate {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let object = self.object.update(cfg, classes, function)?;
        cfg.fail_if_int(object);

        let field_id = if let Some(id) = classes.get_field_id(&self.field) {
            id
        } else {
            return Err(format!("Field not found"));
        };

        let value = self.value.update(cfg, classes, function)?;
        let field_map = cfg.add(Get::from(object, Value::from_raw(1).into()).into());
        let field_offset = cfg.add(Get::from(field_map, Value::from_raw(field_id).into()).into());

        cfg.add_placed(
            Place::None,
            Set::from(object, field_offset.into(), value.into()).into(),
        );

        Ok(value)
    }
}
