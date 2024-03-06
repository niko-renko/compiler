pub trait Update<T> {
    fn update(&self, item: &mut T) -> Result<(), String>;
}
