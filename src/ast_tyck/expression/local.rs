use super::*;

impl Check for Local {
    fn check(&self, _: &Classes, _: &Functions, current: &FunctionContext) -> Result<Type, String> {
        let local_id = match current.get_local_id(self) {
            Some(local_id) => local_id,
            None => return Err(String::from("Local variable not found")),
        };

        let declaration = current.get_local_declaration(local_id).unwrap();
        Ok(declaration.get_type().clone())
    }
}
