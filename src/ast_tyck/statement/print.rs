use super::*;

impl Check for Print {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        if self.get_expression().check(context)? != Type::Int {
            return Err(String::from("Print expression input must be integer"));
        }

        Ok(Type::Void)
    }
}
