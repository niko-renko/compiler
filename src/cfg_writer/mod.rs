use crate::ast_extract::{Classes, FunctionContext};
use crate::cfg::*;

mod bb;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod traits;
mod value;

use traits::Write;

pub struct Writer<'space> {
    static_space: &'space mut String,
    code_space: &'space mut String,
}

impl<'space> Writer<'space> {
    pub fn from(static_space: &'space mut String, code_space: &'space mut String) -> Self {
        Self {
            static_space,
            code_space,
        }
    }

    pub fn write(
        &mut self,
        cfg: &CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // Write static

        // Write code
        for label in cfg {
            let block = cfg.get_block(label);
            label.write(self, classes, function)?;

            if label.get_id() == 0 {
                // write!(writer, "{}:\n", function.get_params_sig())?;
            } else {
                self.code_space.push_str(":\n");
            }

            block.write(self, classes, function)?;
        }

        Ok(())
    }
}
