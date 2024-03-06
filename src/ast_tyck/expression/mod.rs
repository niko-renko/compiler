use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod null;
mod op;

impl Check for Expression {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        match self {
            Expression::Call(e) => e.check(classes, functions, current),
            Expression::Constant(e) => e.check(classes, functions, current),
            Expression::FieldRead(e) => e.check(classes, functions, current),
            Expression::Local(e) => e.check(classes, functions, current),
            Expression::New(e) => e.check(classes, functions, current),
            Expression::Null(e) => e.check(classes, functions, current),
            Expression::Op(e) => e.check(classes, functions, current),
        }
    }
}
