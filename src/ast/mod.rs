use crate::ast_extract::{Classes, Function};
use crate::cfg::{self, *};

mod class;
mod expression;
mod main_function;
mod name;
mod statement;
mod traits;

pub use class::{Class, Field, Method};
use expression::Expression;
pub use expression::Local;
use main_function::MainFunction;
pub use name::Name;
pub use statement::Statement;
use traits::Parse;
pub use traits::Update;

#[derive(Debug)]
pub struct AST {
    classes: Vec<Class>,
    main: MainFunction,
}

impl AST {
    pub fn try_from(program: String) -> Result<Self, String> {
        let (next, classes) = Class::parse_while(program.as_str())?;
        let (next, main) = MainFunction::parse(next, true)?;

        if next.chars().count() > 0 {
            return Err(String::from("Expected end of file"));
        }

        Ok(AST { classes, main })
    }

    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn get_main(&self) -> &MainFunction {
        &self.main
    }
}
