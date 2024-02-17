use super::*;

#[derive(Debug)]
pub struct New {
    class_name: Name,
}

impl Parse for New {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "@", false)?;
        let (next, class_name) = Name::parse(next, false)?;
        Ok((next, New { class_name }))
    }
}
