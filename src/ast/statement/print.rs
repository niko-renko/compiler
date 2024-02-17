use super::*;

#[derive(Debug)]
pub struct Print {
    expression: Expression,
}

impl Parse for Print {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "print(", false)?;
        let (next, expression) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ")", true)?;
        Ok((next, Print { expression }))
    }
}
