pub trait Extract<'from, T, C>
where
    Self: Sized,
{
    fn extract(from: &'from T, context: Option<C>) -> Result<Self, String>;
}
