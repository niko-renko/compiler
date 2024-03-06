use super::*;

impl Check for Local {
    fn check(
        &self,
        classes: &Classes,
        _: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        let local_id = match current.get_local_id(self) {
            Some(local_id) => local_id,
            None => return Err(String::from("Local variable not found")),
        };

        let declaration = current.get_local_declaration(local_id).unwrap();

        if let Type::Object(class_name) = declaration.get_type() {
            if classes.get_class_id(class_name).is_none() {
                return Err(String::from("Class does not exist"));
            }
        }

        Ok(declaration.get_type().clone())
    }
}
