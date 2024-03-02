use super::*;

#[derive(Debug)]
pub struct Function {
    name: Name,
    params: Vec<Declaration>,
    locals: Vec<Declaration>,
    body: Vec<Statement>,
    return_type: Type,
}

impl Function {
    pub fn get_name(&self) -> &Name {
        &self.name
    }

    pub fn get_params(&self) -> &Vec<Declaration> {
        &self.params
    }

    pub fn get_locals(&self) -> &Vec<Declaration> {
        &self.locals
    }

    pub fn get_body(&self) -> &Vec<Statement> {
        &self.body
    }
}

impl Function
where
    Function: Parse,
{
    fn parse_method(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "method", false)?;
        let (next, name) = Name::parse(next, true)?;

        let next = Self::consume_string(next, "(", false)?;
        let (next, params) = Declaration::parse_until(next, ")", ",")?;

        let next = Self::consume_string(next, "returning", true)?;
        let (next, return_type) = Type::parse(next, true)?;

        let next = Self::consume_string(next, "with", true)?;
        let next = Self::consume_string(next, "locals", true)?;
        let (next, locals) = Declaration::parse_until(next, ":", ",")?;
        let (next, body) = Statement::parse_while(next)?;

        Ok((
            next,
            Function {
                name,
                body,
                params,
                locals,
                return_type,
            },
        ))
    }

    fn parse_main(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "main", false)?;
        let next = Self::consume_string(next, "with", true)?;
        let (next, locals) = Declaration::parse_until(next, ":", ",")?;
        let (next, body) = Statement::parse_while(next)?;
        let (_, name) = Name::parse("main", false)?;

        Ok((
            next,
            Function {
                name,
                params: vec![],
                locals,
                body,
                return_type: Type::int_new(),
            },
        ))
    }
}

impl Parse for Function {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        if string.starts_with("method") {
            Self::parse_method(string)
        } else {
            Self::parse_main(string)
        }
    }
}
