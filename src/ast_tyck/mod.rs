use crate::ast::*;
use crate::ast_extract::{Classes, FunctionContext, Functions};
use crate::traits::Extract;

mod expression;
mod statement;
mod traits;

use traits::Check;

pub struct TypeCheckContext<'ast> {
    classes: &'ast Classes<'ast>,
    functions: &'ast Functions<'ast>,
}

impl<'ast> TypeCheckContext<'ast> {
    pub fn new(classes: &'ast Classes<'ast>, functions: &'ast Functions<'ast>) -> Self {
        TypeCheckContext { classes, functions }
    }

    pub fn get_classes(&self) -> &'ast Classes<'ast> {
        self.classes
    }

    pub fn get_functions(&self) -> &'ast Functions<'ast> {
        self.functions
    }
}

pub struct TypeCheck;

impl<'ast> Extract<'ast, AST, TypeCheckContext<'ast>> for TypeCheck {
    fn extract(ast: &'ast AST, context: Option<TypeCheckContext>) -> Result<Self, String> {
        let context = match context {
            Some(context) => context,
            None => return Err(String::from("TypeCheckContext is required")),
        };

        let classes = context.get_classes();
        let functions = context.get_functions();

        for function in functions.iter() {
            for statement in function.get_statements() {
                statement.check(classes, functions, function)?;
            }
        }

        Ok(TypeCheck)
    }
}
