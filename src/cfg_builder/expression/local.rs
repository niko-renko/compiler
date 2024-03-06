use super::*;

impl Build for Local {
    fn update<'cfg>(
        &self,
        _: &'cfg mut CFG,
        _: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        Ok(Named::from(function.get_local_id(self).unwrap()).into())
    }
}
