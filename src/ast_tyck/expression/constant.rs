use super::*;

impl<'ast> Check<'ast> for Constant {
    fn check(&self, _: &mut CheckContext) -> Result<Type, String> {
        Ok(Type::Int)
    }
}
