use super::*;

impl Check for IfOnly {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        if self.get_condition().check(context)? != Type::Int {
            return Err(String::from("If condition must be boolean"));
        }
        self.get_body().check(context)?;
        Ok(Type::Void)
    }
}
