use super::*;

impl<'ast> Check<'ast> for Body {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        for statement in self.get_statements() {
            statement.check(context)?;
        }

        Ok(Type::Void)
    }
}
