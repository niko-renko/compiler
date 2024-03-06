use super::*;

impl Check for IfOnly {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        if self.get_condition().check(classes, functions, current)? != Type::Int {
            return Err(String::from("If condition must be boolean"));
        }
        self.get_body().check(classes, functions, current)?;
        Ok(Type::Void)
    }
}
