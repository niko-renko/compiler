use super::*;

pub struct Function<'ast> {
    class_name: Option<&'ast Name>,
    function_name: &'ast Name,
    params: &'ast Vec<Local>,
    locals: Vec<&'ast Local>,
    statements: &'ast Vec<Statement>,
    this: Local,
}

impl<'ast> Function<'ast> {
    pub fn from(
        class_name: Option<&'ast Name>,
        function_name: &'ast Name,
        params: &'ast Vec<Local>,
        locals: &'ast Vec<Local>,
        statements: &'ast Vec<Statement>,
    ) -> Self {
        let this = Local::from(Name::from(String::from("this")));
        let locals: Vec<&Local> = locals.iter().chain(params.iter()).collect();

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
        let mut params: Vec<&Local> = self.params.iter().collect();

        if let Some(_) = self.class_name {
            params.insert(0, &self.this);
        }

        let params: Vec<&str> = params
            .iter()
            .map(|local| local.get_name().as_ref().as_str())
            .collect();

        format!("({})", params.join(", "))
    }

    pub fn get_local_id(&self, local: &Local) -> Option<usize> {
        let this_vec = vec![&self.this];
        let mut locals = this_vec.iter().chain(self.locals.iter());
        locals.position(|&l| l == local)
    }

    pub fn get_local(&self, id: usize) -> Option<&Local> {
        let this_vec = vec![&self.this];
        let locals: Vec<&&Local> = this_vec.iter().chain(self.locals.iter()).collect();
        locals.get(id).map(|&&l| l)
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        self.statements
    }
}
