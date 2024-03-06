use super::*;

impl Check for FieldUpdate {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        let class_name = match self.get_object().check(classes, functions, current)? {
            Type::Object(object) => object,
            _ => return Err(String::from("Method call on non-object")),
        };

        let field = match classes.get_field(&class_name, self.get_field().get_name()) {
            Some(field) => field,
            None => return Err(String::from("Field does not exist")),
        };

        let value = self.get_value().check(classes, functions, current)?;

        if &value != field.get_type() {
            return Err(String::from("Field update type mismatch"));
        }

        Ok(Type::Void)
    }
}
