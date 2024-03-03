use super::*;

pub trait Check {
    fn check(&self, function: &FunctionContext) -> Result<TypeId, String>;
}
