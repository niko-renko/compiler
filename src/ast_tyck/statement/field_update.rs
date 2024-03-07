use super::*;

impl Check for FieldUpdate {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let class_name = match self.get_object().check(context)? {
            Type::Object(object) => object,
            _ => return Err(String::from("Method call on non-object")),
        };

        let field = match context
            .get_classes()
            .get_field(&class_name, self.get_field().get_name())
        {
            Some(field) => field,
            None => return Err(String::from("Field does not exist")),
        };

        let value = self.get_value().check(context)?;

        if &value != field.get_type() {
            return Err(String::from("Field update type mismatch"));
        }

        Ok(Type::Void)
    }
}
