use super::*;

pub trait Extract<'from>
where
    Self: Sized,
{
    fn extract(from: &'from AST) -> Result<Self, String>;
}
