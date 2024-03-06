use super::*;

pub struct FunctionContext<'ast> {
    class: Option<&'ast Class>,
    function: &'ast Function,
    this: Option<Declaration>,
}

impl<'ast> FunctionContext<'ast> {
    pub fn from(class: Option<&'ast Class>, function: &'ast Function) -> Self {
        // TODO: Name dupe check

        let this = if let Some(class) = class {
            let cloned_class_name = Name::from(String::from(class.get_name().as_ref()));
            let declaration = Declaration::from(
                Name::from(String::from("this")),
                Type::object_from(cloned_class_name),
            );
            Some(declaration)
        } else {
            None
        };

        Self {
            class,
            function,
            this,
        }
    }

    fn this_iter(&'ast self) -> std::vec::IntoIter<&'ast Declaration> {
        match &self.this {
            Some(this) => vec![this].into_iter(),
            None => vec![].into_iter(),
        }
    }

    pub fn get_function_name(&self) -> &Name {
        self.function.get_name()
    }

    pub fn get_class_name(&self) -> Option<&Name> {
        self.class.map(|class| class.get_name())
    }

    pub fn get_params(&self) -> Vec<&Declaration> {
        self.this_iter()
            .chain(self.function.get_params().iter())
            .collect()
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        self.function.get_body()
    }

    pub fn get_return_type(&self) -> &Type {
        self.function.get_return_type()
    }
}

impl<'ast> FunctionContext<'ast> {
    fn all_declarations(&self) -> Vec<&Declaration> {
        let params = self.function.get_params();
        let locals = self.function.get_locals();
        let all = self.this_iter().chain(locals.iter()).chain(params.iter());

        all.collect()
    }

    pub fn get_local_id(&self, local: &Local) -> Option<usize> {
        self.all_declarations()
            .iter()
            .enumerate()
            .find(|(_, declaration)| declaration.get_name() == local.get_name())
            .map(|(index, _)| index)
    }

    pub fn get_local_declaration(&self, id: usize) -> Option<&Declaration> {
        self.all_declarations()
            .get(id)
            .map(|declaration| *declaration)
    }
}
