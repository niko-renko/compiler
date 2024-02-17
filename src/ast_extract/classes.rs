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

    fn get(&self, name: &Name) -> Option<usize> {
        self.ids.get(name).map(|id| *id)
    }

    fn count(&self) -> usize {
        self.next_id
    }
}

pub struct Classes<'ast> {
    classes: Vec<&'ast Name>,
    fields: Index<'ast>,
    methods: Index<'ast>,
}

impl<'ast> Classes<'ast> {
    pub fn get_class_id(&self, class_name: &Name) -> Option<usize> {
        self.classes.iter().position(|&c| c == class_name)
    }

    pub fn get_field_id(&self, field_name: &Local) -> Option<usize> {
        self.fields.get(field_name.get_name())
    }

    pub fn get_method_id(&self, method_name: &Name) -> Option<usize> {
        self.methods.get(method_name)
    }

    pub fn get_field_count(&self, class_id: usize) -> usize {
        self.fields.count()
    }

    pub fn get_vtable_name(&self, class_id: usize) -> String {
        format!("{}_vtable", self.classes[class_id].as_ref())
    }

    pub fn get_fieldmap_name(&self, class_id: usize) -> String {
        format!("{}_field_map", self.classes[class_id].as_ref())
    }

    pub fn write<T: std::io::Write>(&self, writer: &mut T) -> Result<(), std::io::Error> {
        for &class_name in &self.classes {
            let class_id = self.get_class_id(class_name).unwrap();
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
                self.get_fieldmap_name(class_id),
                fields_mapping.join(", ")
            )?;

            let methods = self.methods.class_owns.get(class_name).unwrap_or(&empty);
            let mut methods_mapping = vec![String::from("0"); self.methods.next_id];

            for &id in methods {
                let &function_name = self.methods.ids_reverse.get(&id).unwrap();
                methods_mapping[id] = Function::name(Some(class_name), function_name);
            }

            write!(
                writer,
                "global array {}: {{ {} }}\n",
                self.get_vtable_name(class_id),
                methods_mapping.join(", ")
            )?;
        }

        write!(writer, "\n")
    }
}

impl<'ast> Extract<'ast, AST> for Classes<'ast> {
    fn extract(ast: &'ast AST) -> Result<Self, String> {
        let mut classes = HashSet::new();
        let mut fields = Index::new();
        let mut methods = Index::new();

        for class in ast.get_classes() {
            let class_name = class.get_name();

            if classes.contains(class_name) {
                return Err(format!("Class {} already defined", class_name.as_ref()));
            }

            classes.insert(class_name);

            for field in class.get_fields() {
                fields.bind(class_name, field.get_local().get_name());
            }

            for method in class.get_methods() {
                methods.bind(class_name, method.get_name());
            }
        }

        let classes: Vec<&Name> = classes.iter().map(|name| *name).collect();

        Ok(Classes {
            classes,
            methods,
            fields,
        })
    }
}
