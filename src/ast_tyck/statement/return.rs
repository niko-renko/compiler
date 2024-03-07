use super::*;

impl Check for Return {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        if &self.get_expression().check(context)? != context.get_function().get_return_type() {
            return Err(String::from("Return type does not match"));
        }

        Ok(Type::Void)
    }
}
