use super::*;

#[derive(Debug)]
pub struct Constant(usize);

impl Parse for Constant {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let mut remove = 0;
        let mut value: usize = 0;
        let mut chars = string.chars();

        while let Some(c) = chars.next() {
            if !c.is_digit(10) {
                break;
            }

            value = value * 10 + c.to_digit(10).unwrap() as usize;
            remove += 1;
        }

        if remove == 0 {
            return Err(String::from("Expected constant"));
        }

        let next = Self::consume(string, remove)?;
        Ok((next, Constant(value)))
    }
}

impl Update for Constant {
    fn update<'cfg>(&self, cfg: &'cfg mut CFG, _: &Function, _: &Classes) -> Result<Place, String> {
        let i = Alias::from(Value::from(self.0).into());
        Ok(cfg.add(i.into()))
    }
}
