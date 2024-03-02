use super::*;

pub trait Check {
    fn check(&self) -> Result<Type, String>;
}
