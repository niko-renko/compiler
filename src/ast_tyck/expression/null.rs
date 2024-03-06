use super::*;

impl Check for Null {
    fn check(&self, _: &Classes, _: &Functions, _: &FunctionContext) -> Result<Type, String> {
        let ty = self.get_type();

        if !matches!(ty, Type::Object(_)) {
            return Err(String::from("Null non-object type"));
        }

        Ok(ty.clone())
    }
}
