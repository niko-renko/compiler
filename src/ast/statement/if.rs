use super::*;

#[derive(Debug)]
pub struct If {
    condition: Expression,
    true_body: Body,
    false_body: Body,
}

impl If {
    pub fn get_condition(&self) -> &Expression {
        &self.condition
    }

    pub fn get_true_body(&self) -> &Body {
        &self.true_body
    }

    pub fn get_false_body(&self) -> &Body {
        &self.false_body
    }
}

impl Parse for If {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "if", false)?;
        let (next, condition) = Expression::parse(next, true)?;
        let next = Self::consume_string(next, ":", false)?;
        let (next, true_body) = Body::parse(next, true)?;
        let next = Self::consume_string(next, "else", true)?;
        let (next, false_body) = Body::parse(next, true)?;
        Ok((
            next,
            If {
                condition,
                true_body,
                false_body,
            },
        ))
    }
}
