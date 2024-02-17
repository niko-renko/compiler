use super::*;

#[derive(Debug)]
pub struct Field(Local);

impl Field {
    pub fn get_local(&self) -> &Local {
        &self.0
    }
}

impl Parse for Field {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let (next, local) = Local::parse(string, false)?;
        Ok((next, Field(local)))
    }
}
