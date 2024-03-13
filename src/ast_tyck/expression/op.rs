use super::*;

impl<'ast> Check<'ast> for Op {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        if matches!(self.get_operator(), Operator::Eq) {
            return Ok(Type::Int);
        }

        if self.get_left().check(context)? != Type::Int {
            return Err(format!("Left side of operator is not an integer"));
        }

        if self.get_right().check(context)? != Type::Int {
            return Err(format!("Right side of operator is not an integer"));
        }

        Ok(Type::Int)
    }
}
