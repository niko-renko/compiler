use super::*;

pub struct CheckContext<'ast> {
    classes: &'ast Classes<'ast>,
    functions: &'ast Functions<'ast>,
    function: &'ast FunctionContext<'ast>,
    types: HashMap<&'ast Expression, Type>,
}

impl<'ast> CheckContext<'ast> {
    pub fn from(
        classes: &'ast Classes<'ast>,
        functions: &'ast Functions<'ast>,
        function: &'ast FunctionContext<'ast>,
    ) -> Self {
        CheckContext {
            classes,
            functions,
            function,
            types: HashMap::new(),
        }
    }

    pub fn get_classes(&self) -> &'ast Classes<'ast> {
        self.classes
    }

    pub fn get_functions(&self) -> &'ast Functions<'ast> {
        self.functions
    }

    pub fn get_function(&self) -> &'ast FunctionContext<'ast> {
        self.function
    }

    pub fn get_types_mut(&mut self) -> &mut HashMap<&'ast Expression, Type> {
        &mut self.types
    }

    pub fn move_types(self) -> HashMap<&'ast Expression, Type> {
        self.types
    }
}

pub trait Check<'ast> {
    fn check(&'ast self, context: &mut CheckContext<'ast>) -> Result<Type, String>;
}
