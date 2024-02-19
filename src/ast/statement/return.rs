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

impl Update for Return {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let value = self.expression.update(cfg, classes, function)?;
        cfg.fail_if_ptr(value);
        let value = cfg.to_raw(value);
        cfg.end(cfg::Return::from(value.into()).into());
        Ok(Place::None)
    }
}
