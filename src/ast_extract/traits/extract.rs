use super::*;

pub trait Extract<'ast>
where
    Self: Sized,
{
    fn extract(ast: &'ast AST) -> Result<Self, String>;
}
