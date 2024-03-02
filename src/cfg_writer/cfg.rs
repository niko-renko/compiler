use super::*;

impl Write for CFG {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        for label in self {
            let block = self.get_block(label);
            label.write(writer, classes, function);

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

            block.write(writer, classes, function);
        }

        let class_name = if let Some(class_name) = function.get_class_name() {
            class_name
        } else {
            return;
        };

        let class_id = classes.get_class_id(class_name).unwrap();

        let fields = classes.get_fields_by_class(class_id);
        let mut fields_mapping = vec![String::from("0"); classes.get_field_count()];

        for &id in fields {
            let field_position = id + 2;
            fields_mapping[id] = field_position.to_string();
        }

        let static_fields = format!(
            "global array {}: {{ {} }}\n",
            Writer::get_fieldmap_name(class_name.as_ref()),
            fields_mapping.join(", ")
        );

        writer.write_static(static_fields);

        let methods = classes.get_methods_by_class(class_id);
        let mut methods_mapping = vec![String::from("0"); classes.get_method_count()];

        for &id in methods {
            let method_name = classes.get_method_name(id).unwrap();
            methods_mapping[id] =
                Writer::get_label_name(Some(class_name.as_ref()), method_name.as_ref());
        }

        let static_methods = format!(
            "global array {}: {{ {} }}\n",
            Writer::get_vtable_name(class_name.as_ref()),
            methods_mapping.join(", ")
        );

        writer.write_static(static_methods);
    }
}
