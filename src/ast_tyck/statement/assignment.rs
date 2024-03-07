use super::*;

impl Check for Assignment {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let right = self.get_right().check(context)?;

        let left = match self.get_left() {
            AssignmentLeft::Empty => return Ok(Type::Void),
            AssignmentLeft::Local(local) => local,
        };

        if left.check(context)? != right {
            return Err(String::from("Assignment type mismatch"));
        }

        Ok(Type::Void)
    }
}
