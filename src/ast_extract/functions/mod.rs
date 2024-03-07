use super::*;

mod function_context;

pub use function_context::FunctionContext;

pub struct FunctionsContext;

pub struct Functions<'ast> {
    contexts: Vec<FunctionContext<'ast>>,
}

impl Functions<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &FunctionContext> {
        self.contexts.iter()
    }

    pub fn get_function(
        &self,
        class_name: Option<&Name>,
        function_name: &Name,
    ) -> Option<&FunctionContext> {
        self.contexts
            .iter()
            .find(|f| f.get_class_name() == class_name && f.get_function_name() == function_name)
    }
}

impl<'ast> Extract<'ast, AST, FunctionsContext> for Functions<'ast> {
    fn extract(ast: &'ast AST, _: Option<FunctionsContext>) -> Result<Self, String> {
        let mut contexts = vec![FunctionContext::from(None, ast.get_main())];

        for class in ast.get_classes() {
            for function in class.get_methods() {
                contexts.push(FunctionContext::from(Some(class), function));
            }
        }

        Ok(Functions { contexts })
    }
}
