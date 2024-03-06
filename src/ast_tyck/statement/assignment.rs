use super::*;

impl Check for Assignment {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        let right = self.get_right().check(classes, functions, current)?;

        let left = match self.get_left() {
            AssignmentLeft::Empty => return Ok(Type::Void),
            AssignmentLeft::Local(local) => local,
        };

        if left.check(classes, functions, current)? != right {
            return Err(String::from("Assignment type mismatch"));
        }

        Ok(Type::Void)
    }
}
