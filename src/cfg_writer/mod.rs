use crate::ast_extract::{Classes, FunctionContext};
use crate::cfg::*;
use crate::traits::Extract;

mod bb;
mod control_transfer;
mod instruction;
mod label;
mod place;
mod place_value;
mod traits;
mod value;

pub use traits::Write;

pub struct WriterContext<'ast> {
    classes: &'ast Classes<'ast>,
    function: &'ast FunctionContext<'ast>,
}

impl<'ast> WriterContext<'ast> {
    pub fn new(classes: &'ast Classes, function: &'ast FunctionContext) -> Self {
        Self { classes, function }
    }

    pub fn get_classes(&self) -> &'ast Classes<'ast> {
        self.classes
    }

    pub fn get_function(&self) -> &'ast FunctionContext<'ast> {
        self.function
    }
}

pub struct Writer {
    code_space: String,
    static_space: String,
}

impl Writer {
    pub fn get_static_space(&self) -> &String {
        &self.static_space
    }

    pub fn get_code_space(&self) -> &String {
        &self.code_space
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

impl<'cfg, 'ast> Extract<'cfg, CFG, WriterContext<'ast>> for Writer {
    fn extract(cfg: &'cfg CFG, context: Option<WriterContext>) -> Result<Self, String> {
        let context = match context {
            Some(context) => context,
            None => return Err(String::from("No writer context")),
        };

        let classes = context.get_classes();
        let function = context.get_function();

        let mut writer = Self {
            code_space: String::new(),
            static_space: String::new(),
        };

        for label in cfg {
            let block = cfg.get_block(label);
            label.write(&mut writer, classes, function);

            if label.get_id() == 0 {
                let params: Vec<&str> = function
                    .get_params()
                    .iter()
                    .map(|declaration| declaration.get_name().as_ref().as_str())
                    .collect();

                writer.write_code(&format!("({}):\n", params.join(", ")));
            } else {
                writer.write_code(":\n");
            }

            block.write(&mut writer, classes, function);
        }

        let class_name = if let Some(class_name) = function.get_class_name() {
            class_name
        } else {
            return Ok(writer);
        };

        let fields = classes.get_class_field_ids(class_name);
        let mut fields_mapping = vec![String::from("0"); classes.get_field_count()];

        for id in fields {
            let field_position = id + 2;
            fields_mapping[id] = field_position.to_string();
        }

        let static_fields = format!(
            "global array {}: {{ {} }}\n",
            Writer::get_fieldmap_name(class_name.as_ref()),
            fields_mapping.join(", ")
        );

        writer.write_static(static_fields);

        let methods = classes.get_class_method_ids(class_name);
        let mut methods_mapping = vec![String::from("0"); classes.get_method_count()];

        for id in methods {
            let method = classes.get_method_by_id(id).unwrap();
            methods_mapping[id] =
                Writer::get_label_name(Some(class_name.as_ref()), method.get_name().as_ref());
        }

        let static_methods = format!(
            "global array {}: {{ {} }}\n",
            Writer::get_vtable_name(class_name.as_ref()),
            methods_mapping.join(", ")
        );

        writer.write_static(static_methods);
        Ok(writer)
    }
}
