use super::*;

pub trait Check {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String>;
}
