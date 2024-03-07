use super::*;

pub struct CheckContext<'ast> {
    classes: &'ast Classes<'ast>,
    functions: &'ast Functions<'ast>,
    current: &'ast FunctionContext<'ast>,
    expression_types: &'ast mut HashMap<&'ast Expression, Type>,
}

impl<'ast> CheckContext<'ast> {
    pub fn from(
        classes: &'ast Classes<'ast>,
        functions: &'ast Functions<'ast>,
        current: &'ast FunctionContext<'ast>,
        expression_types: &'ast mut HashMap<&'ast Expression, Type>,
    ) -> Self {
        CheckContext {
            classes,
            functions,
            current,
            expression_types,
        }
    }

    pub fn get_classes(&self) -> &'ast Classes<'ast> {
        self.classes
    }

    pub fn get_functions(&self) -> &'ast Functions<'ast> {
        self.functions
    }

    pub fn get_current(&self) -> &'ast FunctionContext<'ast> {
        self.current
    }

    pub fn get_expression_types(&'ast mut self) -> &'ast mut HashMap<&'ast Expression, Type> {
        self.expression_types
    }
}

pub trait Check {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String>;
}
