use super::*;

pub trait Check {
    fn check(&self, functions: &Functions, current: &FunctionContext) -> Result<Type, String>;
}
