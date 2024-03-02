use std::collections::{HashMap, HashSet};

use super::*;

struct Index<'ast> {
    ids: HashMap<&'ast Name, usize>,
    ids_reverse: HashMap<usize, &'ast Name>,
    class_owns: HashMap<usize, Vec<usize>>,
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

    fn bind(&mut self, class_id: usize, name: &'ast Name) {
        let id;

        if !self.ids.contains_key(name) {
            self.ids.insert(name, self.next_id);
            self.ids_reverse.insert(self.next_id, name);
            id = self.next_id;
            self.next_id += 1;
        } else {
            id = *self.ids.get(name).unwrap();
        }

        self.class_owns.entry(class_id).or_insert(vec![]).push(id);
    }

    fn get(&self, name: &Name) -> Option<usize> {
        self.ids.get(name).map(|id| *id)
    }
}

pub struct Classes<'ast> {
    classes: &'ast Vec<Class>,
    fields: Index<'ast>,
    methods: Index<'ast>,
}

impl<'ast> Classes<'ast> {
    pub fn get_field_id(&self, field_name: &Name) -> Option<usize> {
        self.fields.get(field_name)
    }

    pub fn get_field_count(&self) -> usize {
        self.fields.next_id
    }
}

impl<'ast> Classes<'ast> {
    pub fn get_method_id(&self, method_name: &Name) -> Option<usize> {
        self.methods.get(method_name)
    }

    pub fn get_method_name(&self, method_id: usize) -> Option<&'ast Name> {
        self.methods.ids_reverse.get(&method_id).map(|name| *name)
    }

    pub fn get_method_count(&self) -> usize {
        self.methods.next_id
    }
}

impl<'ast> Classes<'ast> {
    pub fn get_class_id(&self, class_name: &Name) -> Option<usize> {
        self.classes.iter().position(|c| c.get_name() == class_name)
    }

    pub fn get_classes(&self) -> Vec<usize> {
        (0..self.classes.len()).collect()
    }

    pub fn get_class(&self, class_id: usize) -> &'ast Class {
        &self.classes[class_id]
    }

    pub fn get_fields_by_class(&self, class_id: usize) -> &Vec<usize> {
        let fields = self.fields.class_owns.get(&class_id).unwrap();
        fields
    }

    pub fn get_methods_by_class(&self, class_id: usize) -> &Vec<usize> {
        let methods = self.methods.class_owns.get(&class_id).unwrap();
        methods
    }
}

impl<'ast> Extract<'ast, AST> for Classes<'ast> {
    fn extract(ast: &'ast AST) -> Result<Self, String> {
        let mut class_names = HashSet::new();
        let mut fields = Index::new();
        let mut methods = Index::new();
        let mut next_class_id = 0;

        for class in ast.get_classes() {
            let class_name = class.get_name();

            if class_names.contains(class_name) {
                return Err(format!("Class {} already defined", class_name.as_ref()));
            }

            class_names.insert(class_name);

            for field in class.get_fields() {
                fields.bind(next_class_id, field.get_local().get_name());
            }

            for method in class.get_methods() {
                methods.bind(next_class_id, method.get_name());
            }

            next_class_id += 1;
        }

        Ok(Classes {
            classes: ast.get_classes(),
            methods,
            fields,
        })
    }
}
