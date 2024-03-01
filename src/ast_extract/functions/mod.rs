use super::*;

mod function;

pub use function::FunctionContext;

pub struct Functions<'ast>(Vec<FunctionContext<'ast>>);

impl<'ast> Extract<'ast, AST> for Functions<'ast> {
    fn extract(ast: &'ast AST) -> Result<Self, String> {
        let mut contexts = vec![FunctionContext::from(None, ast.get_main())];

        for class in ast.get_classes() {
            for function in class.get_methods() {
                contexts.push(FunctionContext::from(Some(class), function));
            }
        }

        Ok(Functions(contexts))
    }
}

impl<'ast> Iterator for Functions<'ast> {
    type Item = FunctionContext<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
