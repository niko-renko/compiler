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

    pub fn get_name(&self) -> String {
        if let Some(class_name) = self.class_name {
            // BAD! Two definitions for how to make function name
            format!("{}_{}", class_name.as_ref(), self.function_name.as_ref())
        } else {
            String::from(self.function_name.as_ref())
        }
    }

    pub fn get_params_sig(&self) -> String {
        let mut params: Vec<&Local> = self.params.iter().collect();

        if let Some(class_name) = self.class_name {
            params.insert(0, &self.this);
        }

        let params: Vec<&str> = params
            .iter()
            .map(|local| local.get_name().as_ref().as_str())
            .collect();

        params.join(", ")
    }

    pub fn get_local_id(&self, local: &Local) -> Option<usize> {
        self.locals.iter().position(|&l| l == local)
    }

    pub fn get_local(&self, id: usize) -> Option<&Local> {
        self.locals.get(id).copied()
    }

    pub fn get_statements(&self) -> &Vec<Statement> {
        self.statements
    }
}
