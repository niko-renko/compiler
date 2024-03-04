use crate::ast::*;
use crate::ast_extract::{FunctionContext, Functions};
use crate::traits::Extract;

mod expression;
mod statement;
mod traits;
mod type_id;

use traits::Check;
use type_id::TypeId;

pub struct TypeCheckContext;

pub struct TypeCheck;

impl<'ast> Extract<'ast, Functions<'ast>, TypeCheckContext> for TypeCheck {
    fn extract(functions: &'ast Functions, _: Option<TypeCheckContext>) -> Result<Self, String> {
        for function in functions.iter() {
            for statement in function.get_statements() {
                statement.check(&function)?;
            }
        }

        Ok(TypeCheck)
    }
}