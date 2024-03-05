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
    fn check(&self, function: &FunctionContext) -> Result<Type, String> {
        match self {
            Statement::Assignment(s) => s.check(function),
            Statement::FieldUpdate(s) => s.check(function),
            Statement::If(s) => s.check(function),
            Statement::IfOnly(s) => s.check(function),
            Statement::Print(s) => s.check(function),
            Statement::Return(s) => s.check(function),
            Statement::While(s) => s.check(function),
        }
    }
}
