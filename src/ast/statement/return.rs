use super::*;

pub struct Return {
    expression: Expression,
}

impl Return {
    pub fn get_expression(&self) -> &Expression {
        &self.expression
    }
}

impl Parse for Return {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "return", false)?;
        let (next, expression) = Expression::parse(next, true)?;
        Ok((next, Return { expression }))
    }
}
