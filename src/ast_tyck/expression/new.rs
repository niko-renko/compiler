use super::*;

impl<'ast> Check<'ast> for New {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let class_name = self.get_class_name();

        if let Some(_) = context.get_classes().get_class_id(class_name) {
            Ok(Type::Object(class_name.clone()))
        } else {
            Err(format!("Class {} does not exist", class_name.as_ref()))
        }
    }
}
