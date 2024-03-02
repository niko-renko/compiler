use super::*;

mod function;

pub use function::FunctionContext;

pub struct FunctionsContext;

pub struct Functions<'ast>(Vec<FunctionContext<'ast>>);

impl Functions<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &FunctionContext> {
        self.0.iter()
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

        Ok(Functions(contexts))
    }
}
