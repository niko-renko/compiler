use super::*;

impl<'ast> Check<'ast> for Assignment {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
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
