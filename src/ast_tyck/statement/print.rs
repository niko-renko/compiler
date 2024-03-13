use super::*;

impl<'ast> Check<'ast> for Print {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        if self.get_expression().check(context)? != Type::Int {
            return Err(String::from("Print expression input must be integer"));
        }

        Ok(Type::Void)
    }
}
