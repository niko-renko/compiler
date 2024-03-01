use super::*;

#[derive(Debug)]
pub struct Body {
    statements: Vec<Statement>,
}

impl Body {
    pub fn get_statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}

impl Parse for Body {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "{", false)?;
        let next = Self::consume_string(next, "\n", true)?;
        let (next, statements) = Statement::parse_until(next, "}", "\n")?;
        Ok((next, Body { statements }))
    }
}
