use super::*;

impl Check for Print {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        if self.get_expression().check(classes, functions, current)? != Type::Int {
            return Err(String::from("Print expression input must be integer"));
        }

        Ok(Type::Void)
    }
}
