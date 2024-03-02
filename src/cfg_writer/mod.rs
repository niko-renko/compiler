use crate::ast_extract::{Classes, FunctionContext};
use crate::cfg::*;

mod bb;
mod cfg;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod traits;
mod value;

pub use traits::Write;

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

    fn write_code(&mut self, code: &str) {
        self.code_space.push_str(code);
    }

    fn write_static(&mut self, statics: String) {
        self.static_space.push_str(&statics);
    }

    fn get_label_name(class_name: Option<&String>, function_name: &String) -> String {
        let mut name = String::new();

        if let Some(class_name) = class_name {
            name.push_str(class_name);
        }

        name.push_str(function_name);
        name
    }

    fn get_vtable_name(class_name: &String) -> String {
        format!("{}_vtable", class_name)
    }

    fn get_fieldmap_name(class_name: &String) -> String {
        format!("{}_fieldmap", class_name)
    }
}
