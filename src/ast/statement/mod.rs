use super::*;

mod assignment;
mod body;
mod field_update;
mod r#if;
mod if_only;
mod print;
mod r#return;
mod r#while;

pub use assignment::{Assignment, AssignmentLeft};
pub use body::Body;
pub use field_update::FieldUpdate;
pub use if_only::IfOnly;
pub use print::Print;
pub use r#if::If;
pub use r#return::Return;
pub use r#while::While;

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    FieldUpdate(FieldUpdate),
    If(If),
    IfOnly(IfOnly),
    While(While),
    Return(Return),
    Print(Print),
}

impl Parse for Statement {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let first: char = string.chars().next().unwrap();

        if string.starts_with("ifonly") {
            let (next, ifonly) = IfOnly::parse(string, false)?;
            return Ok((next, Statement::IfOnly(ifonly)));
        }

        if string.starts_with("if") {
            let (next, if_statement) = If::parse(string, false)?;
            return Ok((next, Statement::If(if_statement)));
        }

        if string.starts_with("while") {
            let (next, while_statement) = While::parse(string, false)?;
            return Ok((next, Statement::While(while_statement)));
        }

        if string.starts_with("return") {
            let (next, return_statement) = Return::parse(string, false)?;
            return Ok((next, Statement::Return(return_statement)));
        }

        if string.starts_with("print") {
            let (next, print) = Print::parse(string, false)?;
            return Ok((next, Statement::Print(print)));
        }

        if first == '!' {
            let (next, field_update) = FieldUpdate::parse(string, false)?;
            return Ok((next, Statement::FieldUpdate(field_update)));
        }

        if first.is_alphabetic() || first == '_' {
            let (next, assignment) = Assignment::parse(string, false)?;
            return Ok((next, Statement::Assignment(assignment)));
        }

        Err(format!("Unexpected start of a statement: {}", string))
    }
}

impl Update for Statement {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        match self {
            Statement::Assignment(s) => s.update(cfg, classes, function),
            Statement::FieldUpdate(s) => s.update(cfg, classes, function),
            Statement::If(s) => s.update(cfg, classes, function),
            Statement::IfOnly(s) => s.update(cfg, classes, function),
            // Statement::While(while_statement) => while_statement.update(cfg, function, classes),
            Statement::Return(s) => s.update(cfg, classes, function),
            Statement::Print(s) => s.update(cfg, classes, function),
            _ => unimplemented!(),
        }
    }
}
