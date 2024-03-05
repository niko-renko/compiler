use std::collections::HashMap;

use crate::ast::*;
use crate::ast_extract::{Classes, FunctionContext, Functions};
use crate::traits::Extract;

mod expression;
mod statement;
mod traits;

use traits::Check;

pub struct TypeCheckContext;

pub struct TypeCheck;

impl<'ast> Extract<'ast, Functions<'ast>, TypeCheckContext> for TypeCheck {
    fn extract(functions: &'ast Functions, _: Option<TypeCheckContext>) -> Result<Self, String> {
        for function in functions.iter() {
            for statement in function.get_statements() {
                statement.check(functions, function)?;
            }
        }

        Ok(TypeCheck)
    }
}
