use super::*;

#[derive(Debug)]
pub struct Method {
    name: Name,
    params: Vec<Local>,
    locals: Vec<Local>,
    body: Vec<Statement>,
}

impl Method {
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

impl Parse for Method {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "method", false)?;
        let (next, name) = Name::parse(next, true)?;

        let next = Self::consume_string(next, "(", false)?;
        let (next, params) = Local::parse_until(next, ")", ",")?;

        let next = Self::consume_string(next, "with", true)?;
        let next = Self::consume_string(next, "locals", true)?;
        let (next, locals) = Local::parse_until(next, ":", ",")?;
        let (next, body) = Statement::parse_while(next)?;

        Ok((
            next,
            Method {
                name,
                body,
                params,
                locals,
            },
        ))
    }
}
