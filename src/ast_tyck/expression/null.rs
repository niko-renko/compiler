use super::*;

impl Check for Null {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        unimplemented!()
    }
}
