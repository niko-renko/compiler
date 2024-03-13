use super::*;

impl<'ast> Check<'ast> for If {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        if self.get_condition().check(context)? != Type::Int {
            return Err(String::from("If condition must be boolean"));
        }
        self.get_true_body().check(context)?;
        self.get_false_body().check(context)?;
        Ok(Type::Void)
    }
}
