use super::*;

#[derive(Hash, Clone)]
pub enum Type {
    Int,
    Object(Name),
}

impl Type {
    pub fn int_new() -> Self {
        Type::Int
    }

    pub fn object_from(name: Name) -> Self {
        Type::Object(name)
    }
}

impl Parse for Type {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        if string.starts_with("int") {
            Ok((&string[3..], Type::Int))
        } else {
            let (next, name) = Name::parse(string, false)?;
            Ok((next, Type::Object(name)))
        }
    }
}
