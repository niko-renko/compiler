use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod null;
mod op;

impl Check for Expression {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        match self {
            Expression::Call(e) => e.check(context),
            Expression::Constant(e) => e.check(context),
            Expression::FieldRead(e) => e.check(context),
            Expression::Local(e) => e.check(context),
            Expression::New(e) => e.check(context),
            Expression::Null(e) => e.check(context),
            Expression::Op(e) => e.check(context),
        }
    }
}
