use super::*;

pub struct FunctionContext<'ast> {
    class_name: Option<&'ast Name>,
    function_name: &'ast Name,
    params: &'ast Vec<Declaration>,
    locals: Vec<&'ast Declaration>,
    statements: &'ast Vec<Statement>,
    this: Option<Declaration>,
}

impl<'ast> FunctionContext<'ast> {
    pub fn from(
        class_name: Option<&'ast Name>,
        function_name: &'ast Name,
        params: &'ast Vec<Declaration>,
        locals: &'ast Vec<Declaration>,
        statements: &'ast Vec<Statement>,
    ) -> Self {
        let this = if let Some(class_name) = class_name {
            Some(Declaration::from(
                Name::from(String::from("this")),
                Type::new_object(class_name.clone()),
            ))
        } else {
            None
        };

        let locals: Vec<&Declaration> = locals.iter().chain(params.iter()).collect();

        Self {
            class_name,
            function_name,
            params,
            locals,
            statements,
            this,
        }
    }

    pub fn name(class_name: Option<&'ast Name>, function_name: &'ast Name) -> String {
        if let Some(class_name) = class_name {
            format!("{}_{}", class_name.as_ref(), function_name.as_ref())
        } else {
            String::from(function_name.as_ref())
        }
    }

    pub fn get_name(&self) -> String {
        Self::name(self.class_name, self.function_name)
    }

    pub fn get_params_sig(&self) -> String {
        let mut params: Vec<&Declaration> = self.params.iter().collect();

        if let Some(this) = &self.this {
            params.insert(0, this);
        }

        let params: Vec<&str> = params
            .iter()
            .map(|declaration| declaration.get_name().as_ref().as_str())
            .collect();

        format!("({})", params.join(", "))
    }

    pub fn get_local_id(&self, local: &Local) -> Option<usize> {
        let this_vec = if let Some(this) = &self.this {
            vec![this]
        } else {
            vec![]
        };

        let mut locals = this_vec.iter().chain(self.locals.iter());
        locals.position(|&declaration| declaration.get_name() == local.get_name())
    }

    pub fn get_declaration(&self, id: usize) -> Option<&Declaration> {
        let this_vec = if let Some(this) = &self.this {
            vec![this]
        } else {
            vec![]
        };

        let locals: Vec<&&Declaration> = this_vec.iter().chain(self.locals.iter()).collect();
        locals.get(id).map(|&&l| l)
    }

    pub fn get_this_id(&self) -> Option<usize> {
        if let Some(_) = &self.this {
            Some(0)
        } else {
            None
        }
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        self.statements
    }
}
