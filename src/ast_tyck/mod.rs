use std::collections::HashMap;

use crate::ast::*;
use crate::ast_extract::{Classes, FunctionContext, Functions};
use crate::traits::Extract;

mod expression;
mod statement;
mod traits;

use traits::{Check, CheckContext};

pub struct TypesContext<'ast> {
    classes: &'ast Classes<'ast>,
    functions: &'ast Functions<'ast>,
}

impl<'ast> TypesContext<'ast> {
    pub fn new(classes: &'ast Classes<'ast>, functions: &'ast Functions<'ast>) -> Self {
        TypesContext { classes, functions }
    }

    pub fn get_classes(&self) -> &'ast Classes<'ast> {
        self.classes
    }

    pub fn get_functions(&self) -> &'ast Functions<'ast> {
        self.functions
    }
}

pub struct Types<'ast> {
    expression_types: HashMap<&'ast Expression, Type>,
}

impl Types<'_> {
    pub fn get_type(&self, function: &Name, expression: &Expression) -> Option<&Type> {
        self.expression_types.get(expression)
    }
}

impl<'ast> Extract<'ast, AST, TypesContext<'ast>> for Types<'_> {
    fn extract(_: &'ast AST, context: Option<TypesContext>) -> Result<Self, String> {
        let context = match context {
            Some(context) => context,
            None => return Err(String::from("TypeCheckContext is required")),
        };

        let classes = context.get_classes();
        let functions = context.get_functions();

        for function in functions.iter() {
            let mut function_types = HashMap::new();
            let mut check_context =
                CheckContext::from(classes, functions, function, &mut function_types);

            for statement in function.get_statements() {
                statement.check(&mut check_context)?;
            }
        }

        Ok(Self {
            expression_types: HashMap::new(),
        })
    }
}
