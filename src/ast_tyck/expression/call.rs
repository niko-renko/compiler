use super::*;

impl Check for Call {
    fn check(&self, functions: &Functions, current: &FunctionContext) -> Result<Type, String> {
        let object_type = self.get_object().check(functions, current)?;

        unimplemented!()
    }
}
