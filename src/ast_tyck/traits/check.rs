use super::*;

pub struct CheckContext<'ast> {
    classes: &'ast Classes<'ast>,
    functions: &'ast Functions<'ast>,
    function: &'ast FunctionContext<'ast>,
    function_types: &'ast mut HashMap<&'ast Expression, Type>,
}

impl<'ast> CheckContext<'ast> {
    pub fn from(
        classes: &'ast Classes<'ast>,
        functions: &'ast Functions<'ast>,
        function: &'ast FunctionContext<'ast>,
        function_types: &'ast mut HashMap<&'ast Expression, Type>,
    ) -> Self {
        CheckContext {
            classes,
            functions,
            function,
            function_types,
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

    pub fn get_function_types(&'ast mut self) -> &'ast mut HashMap<&'ast Expression, Type> {
        self.function_types
    }
}

pub trait Check {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String>;
}
