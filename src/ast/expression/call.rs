use super::*;

#[derive(Debug)]
pub struct Call {
    object: Box<Expression>,
    method: Name,
    args: Vec<Expression>,
}

impl Parse for Call {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "^", false)?;
        let (next, object) = Expression::parse(next, false)?;

        let next = Self::consume_string(next, ".", false)?;
        let (next, method) = Name::parse(next, false)?;

        let next = Self::consume_string(next, "(", false)?;
        let (next, args) = Expression::parse_until(next, ")", ",")?;

        Ok((
            next,
            Call {
                object: Box::new(object),
                method,
                args,
            },
        ))
    }
}
