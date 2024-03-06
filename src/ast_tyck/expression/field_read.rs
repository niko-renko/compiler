use super::*;

impl Check for FieldRead {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        let class_name = match self.get_object().check(classes, functions, current)? {
            Type::Object(name) => name,
            _ => return Err("Field access on non-object".to_string()),
        };

        let field_id = match classes.get_field_id(self.get_field().get_name()) {
            Some(field_id) => field_id,
            None => return Err("Field not found".to_string()),
        };

        unimplemented!()
    }
}
