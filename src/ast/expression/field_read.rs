use super::*;

#[derive(PartialEq, Eq, Hash)]
pub struct FieldRead {
    object: Box<Expression>,
    field: Local,
}

impl FieldRead {
    pub fn get_object(&self) -> &Expression {
        &self.object
    }

    pub fn get_field(&self) -> &Local {
        &self.field
    }
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
