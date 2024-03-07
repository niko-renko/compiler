use super::*;

impl Check for Constant {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        Ok(Type::Int)
    }
}
