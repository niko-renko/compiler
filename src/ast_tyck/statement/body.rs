use super::*;

impl Check for Body {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        for statement in self.get_statements() {
            statement.check(context)?;
        }

        Ok(Type::Void)
    }
}
