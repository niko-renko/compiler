use std::collections::HashSet;

use super::*;

pub struct ClassesContext;

pub struct Classes<'ast> {
    classes: HashSet<&'ast Name>,
}

impl Classes<'_> {
    pub fn get_class_id(&self, name: &Name) -> Option<usize> {
        unimplemented!()
    }

    pub fn get_class_name(&self, id: usize) -> Option<&Name> {
        unimplemented!()
    }
}

impl Classes<'_> {
    pub fn get_method(&self, class_name: &Name, method_name: &Name) -> Option<&Function> {
        unimplemented!()
    }

    pub fn get_class_methods(&self, class_name: &Name) -> Vec<&Function> {
        unimplemented!()
    }

    pub fn get_class_method_ids(&self, class_name: &Name) -> Vec<usize> {
        unimplemented!()
    }

    pub fn get_method_id(&self, method_name: &Name) -> Option<usize> {
        unimplemented!()
    }

    pub fn get_method_by_id(&self, method_id: usize) -> Option<&Function> {
        unimplemented!()
    }

    pub fn get_method_count(&self) -> usize {
        unimplemented!()
    }
}

impl Classes<'_> {
    pub fn get_field(&self, class_name: &Name, field_name: &Name) -> Option<&Declaration> {
        unimplemented!()
    }

    pub fn get_class_fields(&self, class_name: &Name) -> Vec<&Declaration> {
        unimplemented!()
    }

    pub fn get_class_field_ids(&self, class_name: &Name) -> Vec<usize> {
        unimplemented!()
    }

    pub fn get_field_id(&self, field_name: &Name) -> Option<usize> {
        unimplemented!()
    }

    pub fn get_field_by_id(&self, field_id: usize) -> Option<&Declaration> {
        unimplemented!()
    }

    pub fn get_field_count(&self) -> usize {
        unimplemented!()
    }
}

impl<'ast> Extract<'ast, AST, ClassesContext> for Classes<'_> {
    fn extract(from: &'ast AST, context: Option<ClassesContext>) -> Result<Self, String> {
        unimplemented!()
    }
}
