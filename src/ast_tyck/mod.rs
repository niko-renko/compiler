use crate::ast::*;
use crate::traits::Extract;

mod traits;

use traits::Check;

pub struct TypeCheckContext;

pub struct TypeCheck;

impl<'ast> Extract<'ast, AST, TypeCheckContext> for TypeCheck {
    fn extract(ast: &'ast AST, _: Option<TypeCheckContext>) -> Result<Self, String> {
        Ok(TypeCheck)
    }
}
