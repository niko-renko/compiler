use super::*;

#[derive(Debug)]
pub struct Body {
    statements: Vec<Statement>,
}

impl Parse for Body {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "{", false)?;
        let next = Self::consume_string(next, "\n", true)?;
        let (next, statements) = Statement::parse_until(next, "}", "\n")?;
        Ok((next, Body { statements }))
    }
}

impl Update for Body {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        for statement in &self.statements {
            statement.update(cfg, classes, function)?;
        }
        Ok(Place::None)
    }
}
