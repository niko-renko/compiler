pub trait Extract<'from, T>
where
    Self: Sized,
{
    fn extract(from: &'from T) -> Result<Self, String>;
}
