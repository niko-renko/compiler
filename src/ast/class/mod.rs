use super::*;

mod field;

pub use field::Field;

#[derive(Debug)]
pub struct Class {
    name: Name,
    fields: Vec<Field>,
    methods: Vec<Function>,
}

impl Class {
    pub fn get_name(&self) -> &Name {
        &self.name
    }

    pub fn get_fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_methods(&self) -> &Vec<Function> {
        &self.methods
    }
}

impl Parse for Class {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let next = Self::consume_string(string, "class", false)?;
        let (next, name) = Name::parse(next, true)?;
        let next = Self::consume_string(next, "[", true)?;
        let next = Self::consume_string(next, "fields", true)?;
        let (next, fields) = Field::parse_until(next, "\n", ",")?;
        let (next, methods) = Function::parse_while(next)?;
        let next = Self::consume_string(next, "]", true)?;

        Ok((
            next,
            Class {
                name,
                fields,
                methods,
            },
        ))
    }
}
