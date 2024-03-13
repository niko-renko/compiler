use super::*;

impl<'ast> Check<'ast> for While {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        if self.get_condition().check(context)? != Type::Int {
            return Err(String::from("While condition must be boolean"));
        }
        self.get_body().check(context)?;
        Ok(Type::Void)
    }
}
