use super::*;

mod assignment;
mod body;
mod field_update;
mod r#if;
mod if_only;
mod print;
mod r#return;
mod r#while;

impl Build for Statement {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        match self {
            Statement::Assignment(s) => s.update(cfg, classes, function),
            Statement::FieldUpdate(s) => s.update(cfg, classes, function),
            Statement::If(s) => s.update(cfg, classes, function),
            Statement::IfOnly(s) => s.update(cfg, classes, function),
            Statement::While(s) => s.update(cfg, classes, function),
            Statement::Return(s) => s.update(cfg, classes, function),
            Statement::Print(s) => s.update(cfg, classes, function),
        }
    }
}
