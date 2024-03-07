use super::*;

impl Check for Op {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        if self.get_left().check(context)? != Type::Int {
            return Err(format!("Left side of operator is not an integer"));
        }

        if self.get_right().check(context)? != Type::Int {
            return Err(format!("Right side of operator is not an integer"));
        }

        Ok(Type::Int)
    }
}
