use super::*;

impl Check for Op {
    fn check(&self, functions: &Functions, current: &FunctionContext) -> Result<Type, String> {
        if self.get_left().check(functions, current)? != Type::Int {
            return Err(format!("Left side of operator is not an integer"));
        }

        if self.get_right().check(functions, current)? != Type::Int {
            return Err(format!("Right side of operator is not an integer"));
        }

        Ok(Type::Int)
    }
}
