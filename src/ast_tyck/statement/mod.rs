use super::*;

mod assignment;
mod body;
mod field_update;
mod r#if;
mod if_only;
mod print;
mod r#return;
mod r#while;

impl<'ast> Check<'ast> for Statement {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String> {
        match self {
            Statement::Assignment(s) => s.check(context),
            Statement::FieldUpdate(s) => s.check(context),
            Statement::If(s) => s.check(context),
            Statement::IfOnly(s) => s.check(context),
            Statement::Print(s) => s.check(context),
            Statement::Return(s) => s.check(context),
            Statement::While(s) => s.check(context),
        }
    }
}
