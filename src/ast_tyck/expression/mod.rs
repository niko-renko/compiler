use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod op;

impl Check for Expression {
    fn check(&self, functions: &Functions, current: &FunctionContext) -> Result<Type, String> {
        match self {
            Expression::Call(e) => e.check(functions, current),
            Expression::Constant(e) => e.check(functions, current),
            Expression::FieldRead(e) => e.check(functions, current),
            Expression::Local(e) => e.check(functions, current),
            Expression::New(e) => e.check(functions, current),
            Expression::Op(e) => e.check(functions, current),
        }
    }
}
