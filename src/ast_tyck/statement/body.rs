use super::*;

impl Check for Body {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        for statement in self.get_statements() {
            statement.check(classes, functions, current)?;
        }

        Ok(Type::Void)
    }
}
