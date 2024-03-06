use super::*;

impl Check for If {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        if self.get_condition().check(classes, functions, current)? != Type::Int {
            return Err(String::from("If condition must be boolean"));
        }
        self.get_true_body().check(classes, functions, current)?;
        self.get_false_body().check(classes, functions, current)?;
        Ok(Type::Void)
    }
}
