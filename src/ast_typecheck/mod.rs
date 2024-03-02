use crate::ast::*;
use crate::ast_extract::Classes;

pub struct TypeChecker<'ast> {
    classes: &'ast Classes<'ast>,
}

trait TypeCheck {
    fn typecheck(&self, classes: &Classes) -> Result<Type, String>;
}

impl<'ast> TypeChecker<'ast> {
    pub fn new(classes: &'ast Classes<'ast>) -> Self {
        TypeChecker { classes }
    }
}

impl TypeCheck for TypeChecker<'_> {
    fn typecheck(&self, classes: &Classes) -> Result<Type, String> {
        unimplemented!()
    }
}
