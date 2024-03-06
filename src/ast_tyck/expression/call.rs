use super::*;

impl Check for Call {
    fn check(&self, functions: &Functions, current: &FunctionContext) -> Result<Type, String> {
        let class_name = match self.get_object().check(functions, current)? {
            Type::Object(object) => object,
            _ => return Err(String::from("Cannot call method on non-object")),
        };

        let function = match functions.get_method(&class_name, self.get_method()) {
            Some(function) => function,
            None => return Err(String::from("Method not found")),
        };

        let args = self
            .get_args()
            .iter()
            .map(|arg| arg.check(functions, current));

        for (param, arg) in function.get_params().iter().zip(args) {
            if param.get_type() != &arg? {
                return Err(String::from("Argument type mismatch"));
            }
        }

        Ok(function.get_return_type().clone())
    }
}
