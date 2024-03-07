use super::*;

impl Check for Local {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let current = context.get_current();

        let local_id = match current.get_local_id(self) {
            Some(local_id) => local_id,
            None => return Err(String::from("Local variable not found")),
        };

        let declaration = current.get_local_declaration(local_id).unwrap();
        let ty = declaration.get_type();

        if let Type::Object(class_name) = ty {
            if let Some(_) = context.get_classes().get_class_id(class_name) {
                Ok(ty.clone())
            } else {
                Err(format!("Class {} does not exist", class_name.as_ref()))
            }
        } else {
            Ok(declaration.get_type().clone())
        }
    }
}
