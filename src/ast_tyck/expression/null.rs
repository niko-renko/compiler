use super::*;

impl Check for Null {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let ty = self.get_type();

        if let Type::Object(class_name) = ty {
            if let Some(_) = context.get_classes().get_class_id(class_name) {
                return Ok(ty.clone());
            }

            Err(format!("Class {} does not exist", class_name.as_ref()))
        } else {
            Err(String::from("Null non-object type"))
        }
    }
}
