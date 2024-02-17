use super::*;

#[derive(Debug)]
pub struct While {
    condition: Expression,
    body: Body,
}

impl Parse for While {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "while", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, body) = Body::parse(next, true)?;
        Ok((next, While { condition, body }))
    }
}