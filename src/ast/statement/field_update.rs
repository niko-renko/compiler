use super::*;

#[derive(Debug)]
pub struct FieldUpdate {
    object: Expression,
    field: Local,
    value: Expression,
}

impl Parse for FieldUpdate {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "!", false)?;
        let (next, object) = Expression::parse(next, false)?;
        let next = Self::consume_string(next, ".", false)?;
        let (next, field) = Local::parse(next, false)?;
        let next = Self::consume_string(next, "=", true)?;
        let (next, value) = Expression::parse(next, true)?;

        Ok((
            next,
            FieldUpdate {
                object,
                field,
                value,
            },
        ))
    }
}
