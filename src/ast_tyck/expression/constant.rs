use super::*;

impl Check for Constant {
    fn check(&self, _: &Functions, _: &FunctionContext) -> Result<Type, String> {
        Ok(Type::Int)
    }
}
