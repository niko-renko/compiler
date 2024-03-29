use super::*;

#[derive(PartialEq, Eq, Hash)]
pub struct Local(Name);

impl From<Name> for Local {
    fn from(name: Name) -> Self {
        Local(name)
    }
}

impl Local {
    pub fn get_name(&self) -> &Name {
        &self.0
    }
}

impl Parse for Local {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let (next, name) = Name::parse(string, false)?;
        Ok((next, Local(name)))
    }
}
