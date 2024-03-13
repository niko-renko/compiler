use super::*;

impl<'ast> Check<'ast> for Return {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        if &self.get_expression().check(context)? != context.get_function().get_return_type() {
            return Err(String::from("Return type does not match"));
        }

        Ok(Type::Void)
    }
}
