mod class;
mod declaration;
mod expression;
mod function;
mod name;
mod statement;
mod traits;
mod r#type;

pub use class::Class;
pub use declaration::Declaration;
pub use expression::*;
pub use function::Function;
pub use name::Name;
pub use r#type::Type;
pub use statement::*;
use traits::Parse;

#[derive(Debug)]
pub struct AST {
    classes: Vec<Class>,
    main: Function,
}

impl AST {
    pub fn try_from(program: String) -> Result<Self, String> {
        let (next, classes) = Class::parse_while(program.as_str())?;
        let (next, main) = Function::parse(next, true)?;

        if next.chars().count() > 0 {
            return Err(format!("Unexpected end of file: {}", next));
        }

        Ok(AST { classes, main })
    }

    pub fn get_classes(&self) -> &Vec<Class> {
        &self.classes
    }

    pub fn get_main(&self) -> &Function {
        &self.main
    }
}
