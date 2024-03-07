use super::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Constant(usize);

impl Constant {
    pub fn get_value(&self) -> usize {
        self.0
    }
}

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
