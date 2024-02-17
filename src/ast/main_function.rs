use super::*;

#[derive(Debug)]
pub struct MainFunction {
    name: Name,
    params: Vec<Local>,
    locals: Vec<Local>,
    body: Vec<Statement>,
}

impl MainFunction {
    pub fn get_name(&self) -> &Name {
        &self.name
    }

    pub fn get_params(&self) -> &Vec<Local> {
        &self.params
    }

    pub fn get_locals(&self) -> &Vec<Local> {
        &self.locals
    }

    pub fn get_body(&self) -> &Vec<Statement> {
        &self.body
    }
}

impl Parse for MainFunction {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "main", false)?;
        let next = Self::consume_string(next, "with", true)?;
        let (next, locals) = Local::parse_until(next, ":", ",")?;
        let (next, body) = Statement::parse_while(next)?;
        let (_, name) = Name::parse("main", false)?;

        Ok((
            next,
            MainFunction {
                name,
                params: vec![],
                locals,
                body,
            },
        ))
    }
}
