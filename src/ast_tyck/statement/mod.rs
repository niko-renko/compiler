use super::*;

mod assignment;
mod body;
mod field_update;
mod r#if;
mod if_only;
mod print;
mod r#return;
mod r#while;

impl Check for Statement {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        match self {
            Statement::Assignment(s) => s.check(classes, functions, current),
            Statement::FieldUpdate(s) => s.check(classes, functions, current),
            Statement::If(s) => s.check(classes, functions, current),
            Statement::IfOnly(s) => s.check(classes, functions, current),
            Statement::Print(s) => s.check(classes, functions, current),
            Statement::Return(s) => s.check(classes, functions, current),
            Statement::While(s) => s.check(classes, functions, current),
        }
    }
}
