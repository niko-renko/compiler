use super::*;

impl<'ast> Check<'ast> for FieldRead {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        let class_name = match self.get_object().check(context)? {
            Type::Object(name) => name,
            _ => return Err(String::from("Field access on non-object")),
        };

        let field = match context
            .get_classes()
            .get_field(&class_name, self.get_field().get_name())
        {
            Some(field) => field,
            None => return Err(String::from("Field does not exist")),
        };

        Ok(field.get_type().clone())
    }
}
