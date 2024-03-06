use std::collections::{HashMap, HashSet};

use super::*;

struct ClassIndex<'ast, T> {
    next_id: usize,
    ids: HashMap<&'ast Name, usize>,
    ids_reverse: HashMap<usize, &'ast Name>,
    class_owns_ids: HashMap<usize, Vec<usize>>,
    class_owns_items: HashMap<usize, Vec<&'ast T>>,
}

impl<'ast, T> ClassIndex<'ast, T> {
    fn new() -> Self {
        ClassIndex {
            next_id: 0,
            ids: HashMap::new(),
            ids_reverse: HashMap::new(),
            class_owns_ids: HashMap::new(),
            class_owns_items: HashMap::new(),
        }
    }

    fn bind(&mut self, class_id: usize, item: &'ast T, item_name: &'ast Name) {
        let id;

        if !self.ids.contains_key(item_name) {
            self.ids.insert(item_name, self.next_id);
            self.ids_reverse.insert(self.next_id, item_name);
            id = self.next_id;
            self.next_id += 1;
        } else {
            id = *self.ids.get(item_name).unwrap();
        }

        self.class_owns_ids
            .entry(class_id)
            .or_insert(vec![])
            .push(id);

        self.class_owns_items
            .entry(class_id)
            .or_insert(vec![])
            .push(item);
    }
}

pub struct ClassesContext;

pub struct Classes<'ast> {
    classes: Vec<&'ast Name>,
    methods: ClassIndex<'ast, Function>,
    fields: ClassIndex<'ast, Declaration>,
}

impl Classes<'_> {
    pub fn get_class_id(&self, name: &Name) -> Option<usize> {
        self.classes.iter().position(|&n| n == name)
    }

    pub fn get_class_name(&self, id: usize) -> Option<&Name> {
        self.classes.get(id).map(|n| *n)
    }
}

impl Classes<'_> {
    pub fn get_method_id(&self, method_name: &Name) -> Option<usize> {
        self.methods.ids.get(method_name).map(|id| *id)
    }

    pub fn get_method_name(&self, id: usize) -> Option<&Name> {
        self.methods.ids_reverse.get(&id).map(|name| *name)
    }

    pub fn get_class_method_ids(&self, class_name: &Name) -> Option<&Vec<usize>> {
        let class_id = self.get_class_id(class_name).unwrap();
        self.methods.class_owns_ids.get(&class_id)
    }

    pub fn get_method_count(&self) -> usize {
        self.methods.next_id
    }
}

impl Classes<'_> {
    pub fn get_field_id(&self, field_name: &Name) -> Option<usize> {
        self.fields.ids.get(field_name).map(|id| *id)
    }

    pub fn _get_field_name(&self, id: usize) -> Option<&Name> {
        self.fields.ids_reverse.get(&id).map(|name| *name)
    }

    pub fn get_class_field_ids(&self, class_name: &Name) -> Option<&Vec<usize>> {
        let class_id = self.get_class_id(class_name).unwrap();
        self.fields.class_owns_ids.get(&class_id)
    }

    pub fn get_field_count(&self) -> usize {
        self.fields.next_id
    }
}

impl Classes<'_> {
    pub fn get_field(&self, class_name: &Name, field_name: &Name) -> Option<&Declaration> {
        let class_id = self.get_class_id(class_name)?;

        self.fields
            .class_owns_items
            .get(&class_id)
            .unwrap()
            .iter()
            .find(|field| field.get_name() == field_name)
            .map(|field| *field)
    }
}

impl<'ast> Extract<'ast, AST, ClassesContext> for Classes<'ast> {
    fn extract(ast: &'ast AST, _: Option<ClassesContext>) -> Result<Self, String> {
        let mut classes = vec![];
        let mut classes_set = HashSet::new();
        let mut next_id = 0;

        let mut methods = ClassIndex::new();
        let mut fields = ClassIndex::new();

        for class in ast.get_classes() {
            if classes_set.contains(class.get_name()) {
                return Err(String::from("Class already defined"));
            }

            classes_set.insert(class.get_name());
            classes.push(class.get_name());

            for method in class.get_methods() {
                methods.bind(next_id, method, method.get_name());
            }

            for field in class.get_fields() {
                fields.bind(next_id, field, field.get_name());
            }

            next_id += 1;
        }

        let classes = Self {
            classes: classes_set.into_iter().collect(),
            methods,
            fields,
        };

        Ok(classes)
    }
}
