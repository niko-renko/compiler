use super::*;

impl Check for Call {
    fn check(
        &self,
        classes: &Classes,
        functions: &Functions,
        current: &FunctionContext,
    ) -> Result<Type, String> {
        let class_name = match self.get_object().check(classes, functions, current)? {
            Type::Object(object) => object,
            _ => return Err(String::from("Cannot call method on non-object")),
        };

        let function_context = functions.iter().find(|f| {
            f.get_class_name() == Some(&class_name) && f.get_function_name() == self.get_method()
        });

        let function_context = match function_context {
            Some(function_context) => function_context,
            None => return Err(String::from("Method not found")),
        };

        for (param, arg) in function_context.get_params().iter().zip(self.get_args()) {
            let arg_type = arg.check(classes, functions, current)?;
            if param.get_type() != &arg_type {
                return Err(String::from("Argument type mismatch"));
            }
        }

        Ok(function_context.get_return_type().clone())
    }
}
