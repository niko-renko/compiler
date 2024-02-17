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

impl Update for Print {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        let place_value = self.expression.update(cfg, classes, function)?;
        cfg.fail_if_ptr(place_value);
        let raw = cfg.to_raw(place_value);
        cfg.add_placed(Place::None, cfg::Print::from(raw.into()).into());
        Ok(place_value)
    }
}
