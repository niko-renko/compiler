use super::*;

mod function;

pub use function::Function;

pub struct Functions<'ast>(Vec<Function<'ast>>);

impl<'ast> Extract<'ast> for Functions<'ast> {
    fn extract(ast: &'ast AST) -> Result<Self, String> {
        let mut functions = vec![];

        let main = ast.get_main();
        functions.push(Function::from(
            None,
            main.get_name(),
            main.get_params(),
            main.get_locals(),
            main.get_body(),
        ));

        for class in ast.get_classes() {
            for method in class.get_methods() {
                functions.push(Function::from(
                    Some(class.get_name()),
                    method.get_name(),
                    method.get_params(),
                    method.get_locals(),
                    method.get_body(),
                ))
            }
        }

        Ok(Functions(functions))
    }
}

impl<'ast> Iterator for Functions<'ast> {
    type Item = Function<'ast>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
