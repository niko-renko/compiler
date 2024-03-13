use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod null;
mod op;

impl<'ast> Check<'ast> for Expression {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        let expression_type = match self {
            Expression::Call(e) => e.check(context),
            Expression::Constant(e) => e.check(context),
            Expression::FieldRead(e) => e.check(context),
            Expression::Local(e) => e.check(context),
            Expression::New(e) => e.check(context),
            Expression::Null(e) => e.check(context),
            Expression::Op(e) => e.check(context),
        }?;

        context
            .get_types_mut()
            .insert(self, expression_type.clone());

        Ok(expression_type)
    }
}
