use crate::ast::{self, *};
use crate::ast_extract::{Classes, FunctionContext};
use crate::ast_tyck::Types;
use crate::cfg::{self, *};
use crate::traits::Update;

mod expression;
mod statement;
mod traits;

use traits::Build;

pub struct Builder<'ast> {
    classes: &'ast Classes<'ast>,
    types: &'ast Types<'ast>,
    function: &'ast FunctionContext<'ast>,
}

impl<'ast> Builder<'ast> {
    pub fn from(
        classes: &'ast Classes,
        types: &'ast Types,
        function: &'ast FunctionContext,
    ) -> Self {
        Builder {
            classes,
            types,
            function,
        }
    }
}

impl<'ast> Update<CFG> for Builder<'ast> {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        for statement in self.function.get_statements() {
            statement.update(cfg, self.classes, self.types, self.function)?;
        }

        Ok(())
    }
}
