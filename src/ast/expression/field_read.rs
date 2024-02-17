use super::*;

#[derive(Debug)]
pub struct FieldRead {
    object: Box<Expression>,
    field: Local,
}

impl Parse for FieldRead {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "&", false)?;
        let (next, object) = Expression::parse(next, false)?;

        let next = Self::consume_string(next, ".", false)?;
        let (next, field) = Local::parse(next, false)?;

        Ok((
            next,
            FieldRead {
                object: Box::new(object),
                field,
            },
        ))
    }
}
