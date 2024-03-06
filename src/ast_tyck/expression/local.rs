use super::*;

impl Check for Local {
    fn check(&self, _: &Classes, _: &Functions, current: &FunctionContext) -> Result<Type, String> {
        let declaration_id = match current.get_declaration_id(self) {
            Some(declaration) => declaration,
            None => {
                return Err(format!(
                    "Local variable not found: {}",
                    self.get_name().as_ref()
                ))
            }
        };

        let decalaration = current.get_declaration(declaration_id).unwrap();
        Ok(decalaration.get_type().clone())
    }
}
