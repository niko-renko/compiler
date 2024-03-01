use super::*;
use crate::ast::{self, *};
use crate::ast_extract::{Classes, FunctionContext};
use crate::cfg::{self, *};

mod expression;
mod statement;
mod traits;

use traits::Build;

pub struct Builder<'ast> {
    classes: &'ast Classes<'ast>,
    function: &'ast FunctionContext<'ast>,
}

impl<'ast> Builder<'ast> {
    pub fn from(classes: &'ast Classes, function: &'ast FunctionContext) -> Self {
        Builder { classes, function }
    }
}

impl<'ast> Update for Builder<'ast> {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        for statement in self.function.get_statements() {
            statement.update(cfg, self.classes, self.function)?;
        }

        Ok(())
    }
}
