use super::*;

impl Check for Local {
    fn check(&self, function: &FunctionContext) -> Result<Type, String> {
        let declaration_id = match function.get_declaration_id(self) {
            Some(declaration) => declaration,
            None => {
                return Err(format!(
                    "Local variable not found: {}",
                    self.get_name().as_ref()
                ))
            }
        };

        let decalaration = function.get_declaration(declaration_id).unwrap();
        unimplemented!()
    }
}
