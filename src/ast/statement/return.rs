use super::*;

#[derive(Debug)]
pub struct Return {
    expression: Expression,
}

impl Parse for Return {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "return", false)?;
        let (next, expression) = Expression::parse(next, true)?;
        Ok((next, Return { expression }))
    }
}
