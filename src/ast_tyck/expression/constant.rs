use super::*;

impl Check for Constant {
    fn check(&self, _: &FunctionContext) -> Result<Type, String> {
        Ok(Type::int_new())
    }
}
