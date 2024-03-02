use super::*;

impl Build for Local {
    fn update<'cfg>(
        &self,
        _: &'cfg mut CFG,
        _: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        if let Some(local_id) = function.get_declaration_id(self) {
            Ok(Named::from(local_id).into())
        } else {
            Err(format!("Local {} not found", self.get_name().as_ref()))
        }
    }
}
