use super::*;

impl Check for New {
    fn check(&self, functions: &Functions, _: &FunctionContext) -> Result<Type, String> {
        let class_name = self.get_class_name();

        if functions.class_exists(class_name) {
            Ok(Type::Object(class_name.clone()))
        } else {
            Err(format!("Class {} does not exist", class_name.as_ref()))
        }
    }
}
