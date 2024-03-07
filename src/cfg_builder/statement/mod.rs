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
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        match self {
            Statement::Assignment(s) => s.update(cfg, classes, types, function),
            Statement::FieldUpdate(s) => s.update(cfg, classes, types, function),
            Statement::If(s) => s.update(cfg, classes, types, function),
            Statement::IfOnly(s) => s.update(cfg, classes, types, function),
            Statement::While(s) => s.update(cfg, classes, types, function),
            Statement::Return(s) => s.update(cfg, classes, types, function),
            Statement::Print(s) => s.update(cfg, classes, types, function),
        }
    }
}
