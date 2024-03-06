use super::*;

impl Check for Return {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        if &self.get_expression().check(classes, functions, current)? != current.get_return_type() {
            return Err(String::from(
                "Return type does not match function return type",
            ));
        }

        Ok(Type::Void)
    }
}
