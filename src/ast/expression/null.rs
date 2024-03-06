use super::*;

pub struct Null {
    ty: Type,
}

impl Parse for Null {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "null:", false)?;
        let (next, ty) = Type::parse(next, false)?;
        Ok((next, Self { ty }))
    }
}
