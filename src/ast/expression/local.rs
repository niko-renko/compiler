use super::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Local(Name);

impl Local {
    pub fn from(name: Name) -> Self {
        Local(name)
    }

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

impl Update for Local {
    fn update<'cfg>(
        &self,
        _: &'cfg mut CFG,
        _: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        if let Some(local_id) = function.get_local_id(self) {
            Ok(Named::from(local_id).into())
        } else {
            Err(format!("Local {} not found", self.get_name().as_ref()))
        }
    }
}
