use super::*;

impl Check for Call {
    fn check(&self, context: &mut CheckContext) -> Result<Type, String> {
        let class_name = match self.get_object().check(context)? {
            Type::Object(object) => object,
            _ => return Err(String::from("Method call on non-object")),
        };

        let function_context = match context
            .get_functions()
            .get_function(Some(&class_name), self.get_method())
        {
            Some(function_context) => function_context,
            None => return Err(String::from("Method not found")),
        };

        let mut params = function_context.get_params();
        if let Some(_) = function_context.get_class_name() {
            params = params[1..].to_vec();
        }

        for (param, arg) in params.iter().zip(self.get_args()) {
            let arg_type = arg.check(context)?;
            if param.get_type() != &arg_type {
                return Err(String::from("Argument type mismatch"));
            }
        }

        Ok(function_context.get_return_type().clone())
    }
}
