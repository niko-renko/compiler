use super::*;

impl Check for If {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        if self.get_condition().check(context)? != Type::Int {
            return Err(String::from("If condition must be boolean"));
        }
        self.get_true_body().check(context)?;
        self.get_false_body().check(context)?;
        Ok(Type::Void)
    }
}
