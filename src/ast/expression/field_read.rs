use super::*;

#[derive(Debug)]
pub struct FieldRead {
    object: Box<Expression>,
    field: Local,
}

impl Parse for FieldRead {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "&", false)?;
        let (next, object) = Expression::parse(next, false)?;

        let next = Self::consume_string(next, ".", false)?;
        let (next, field) = Local::parse(next, false)?;

        Ok((
            next,
            FieldRead {
                object: Box::new(object),
                field,
            },
        ))
    }
}

impl Update for FieldRead {
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

        let field_map = cfg.add(Get::from(object.into(), Value::from_raw(1).into()).into());
        let field_offset =
            cfg.add(Get::from(field_map.into(), Value::from_raw(field_id).into()).into());
        cfg.fail_if(field_offset.clone(), false, FailReason::NoSuchField);

        let field_value = cfg.add(Get::from(object.into(), field_offset.into()).into());
        Ok(field_value)
    }
}
