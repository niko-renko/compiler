use super::*;

impl Check for New {
    fn check(&self, classes: &Classes, _: &Functions, _: &FunctionContext) -> Result<Type, String> {
        let class_name = self.get_class_name();

        if let Some(_) = classes.get_class_id(class_name) {
            Ok(Type::Object(class_name.clone()))
        } else {
            Err(format!("Class {} does not exist", class_name.as_ref()))
        }
    }
}
