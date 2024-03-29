use super::*;

pub struct IfOnly {
    condition: Expression,
    body: Body,
}

impl IfOnly {
    pub fn get_condition(&self) -> &Expression {
        &self.condition
    }

    pub fn get_body(&self) -> &Body {
        &self.body
    }
}

impl Parse for IfOnly {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "ifonly", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, body) = Body::parse(next, true)?;

        Ok((next, IfOnly { condition, body }))
    }
}
