use super::*;

#[derive(Debug)]
pub struct Call {
    object: Box<Expression>,
    method: Name,
    args: Vec<Expression>,
}

impl Parse for Call {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "^", false)?;
        let (next, object) = Expression::parse(next, false)?;

        let next = Self::consume_string(next, ".", false)?;
        let (next, method) = Name::parse(next, false)?;

        let next = Self::consume_string(next, "(", false)?;
        let (next, args) = Expression::parse_until(next, ")", ",")?;

        Ok((
            next,
            Call {
                object: Box::new(object),
                method,
                args,
            },
        ))
    }
}

impl Update for Call {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let object = self.object.update(cfg, classes, function)?;
        cfg.fail_if_int(object);

        let mut args = vec![];
        for arg in &self.args {
            args.push(arg.update(cfg, classes, function)?.into());
        }

        let method_id = if let Some(id) = classes.get_method_id(&self.method) {
            id
        } else {
            return Err(format!("Method {} not found", function.get_name()));
        };

        let vtable = cfg.add(Get::from(object, Value::from_raw(0).into()).into());
        let method = cfg.add(Get::from(vtable, Value::from_raw(method_id).into()).into());

        cfg.fail_if(method, false, FailReason::NoSuchMethod);
        Ok(cfg.add(cfg::Call::from(method, object, args).into()))
    }
}
