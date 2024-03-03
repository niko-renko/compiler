use super::*;

pub struct Declaration {
    name: Name,
    ty: Type,
}

impl Declaration {
    pub fn from(name: Name, ty: Type) -> Self {
        Self { name, ty }
    }

    pub fn get_name(&self) -> &Name {
        &self.name
    }
}

impl Parse for Declaration {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let (next, name) = Name::parse(string, false)?;
        let next = Self::consume_string(next, ":", true)?;
        let (next, ty) = Type::parse(next, false)?;

        Ok((next, Declaration { name, ty }))
    }
}
