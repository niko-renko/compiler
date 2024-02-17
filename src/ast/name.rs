use lazy_static::lazy_static;

use std::collections::HashSet;

use super::*;

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("method");
        set.insert("if");
        set.insert("ifonly");
        set.insert("while");
        set.insert("with");
        set.insert("locals");
        set.insert("fields");
        set.insert("print");
        set.insert("return");
        set
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Name {
    pub fn from(name: String) -> Self {
        Name(name)
    }
}

impl AsRef<String> for Name {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Parse for Name {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let length = string.chars().count();
        let end = string.find(|c: char| !c.is_alphabetic()).unwrap_or(length);
        let name = &string[0..end];

        if KEYWORDS.contains(name) {
            return Err(format!("{} is a keyword", string));
        }

        let name = String::from(name);
        let next = Self::consume(string, end)?;
        Ok((next, Name(name)))
    }
}
