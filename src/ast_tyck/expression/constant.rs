use super::*;

impl Check for Constant {
    fn check(&self, _: &FunctionContext) -> Result<TypeId, String> {
        Ok(TypeId::from(Type::Int))
    }
}
