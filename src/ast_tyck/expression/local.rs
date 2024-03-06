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
        let ty = declaration.get_type();

        if let Type::Object(class_name) = ty {
            if let Some(_) = classes.get_class_id(class_name) {
                Ok(ty.clone())
            } else {
                Err(format!("Class {} does not exist", class_name.as_ref()))
            }
        } else {
            Ok(declaration.get_type().clone())
        }
    }
}
