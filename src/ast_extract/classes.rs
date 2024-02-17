use std::collections::{HashMap, HashSet};

use super::*;
struct Index<'ast> {
    ids: HashMap<&'ast Name, usize>,
    ids_reverse: HashMap<usize, &'ast Name>,
    class_owns: HashMap<&'ast Name, Vec<usize>>,
    next_id: usize,
}

impl<'ast> Index<'ast> {
    fn new() -> Self {
        Index {
            ids: HashMap::new(),
            ids_reverse: HashMap::new(),
            class_owns: HashMap::new(),
            next_id: 0,
        }
    }

    fn bind(&mut self, class_name: &'ast Name, name: &'ast Name) {
        let id;

        if !self.ids.contains_key(name) {
            self.ids.insert(name, self.next_id);
            self.ids_reverse.insert(self.next_id, name);
            id = self.next_id;
            self.next_id += 1;
        } else {
            id = *self.ids.get(name).unwrap();
        }

        if !self.class_owns.contains_key(class_name) {
            self.class_owns.insert(class_name, Vec::new());
        }

        self.class_owns.get_mut(class_name).unwrap().push(id);
    }
}

pub struct Classes<'ast> {
    classes: HashSet<&'ast Name>,
    fields: Index<'ast>,
    methods: Index<'ast>,
}

impl<'ast> Classes<'ast> {
    pub fn get_field_count(class_name: &Name) -> usize {
        unimplemented!()
    }

    pub fn get_vtable_name(&self, class_name: &Name) -> String {
        format!("{}_vtable", class_name.as_ref())
    }

    pub fn get_fieldmap_name(&self, class_name: &Name) -> String {
        format!("{}_field_map", class_name.as_ref())
    }

    pub fn get_field_id(field_name: &Local) -> usize {
        unimplemented!()
    }

    pub fn get_method_id(method_name: &Name) -> usize {
        unimplemented!()
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        for &class_name in &self.classes {
            let empty = Vec::new();

            let fields = self.fields.class_owns.get(class_name).unwrap_or(&empty);
            let mut fields_mapping = vec![String::from("0"); self.fields.next_id];

            for &id in fields {
                let field_position = id + 2;
                fields_mapping[id] = field_position.to_string();
            }

            write!(
                writer,
                "global array {}: {{ {} }}\n",
                self.get_fieldmap_name(class_name),
                fields_mapping.join(", ")
            )?;

            let methods = self.methods.class_owns.get(class_name).unwrap_or(&empty);
            let mut methods_mapping = vec![String::from("0"); self.methods.next_id];

            for &id in methods {
                let &function_name = self.methods.ids_reverse.get(&id).unwrap();
                // BAD! Two definitions for how to make function name
                methods_mapping[id] = format!("{}_{}", class_name.as_ref(), function_name.as_ref());
            }

            write!(
                writer,
                "global array {}: {{ {} }}\n",
                self.get_vtable_name(class_name),
                methods_mapping.join(", ")
            )?;
        }

        write!(writer, "\n")?;
        Ok(())
    }
}

impl<'ast> Extract<'ast> for Classes<'ast> {
    fn extract(ast: &'ast AST) -> Result<Self, String> {
        let mut classes = Classes {
            classes: HashSet::new(),
            fields: Index::new(),
            methods: Index::new(),
        };

        for class in ast.get_classes() {
            let class_name = class.get_name();

            if classes.classes.contains(class_name) {
                return Err(format!("Class {} already defined", class_name.as_ref()));
            }

            classes.classes.insert(class_name);

            for field in class.get_fields() {
                classes
                    .fields
                    .bind(class_name, field.get_local().get_name());
            }

            for method in class.get_methods() {
                classes.methods.bind(class_name, method.get_name());
            }
        }

        Ok(classes)
    }
}
