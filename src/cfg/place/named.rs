use super::*;

#[derive(Clone, Copy)]
pub struct Named(usize, usize);

impl Named {
    pub fn from(local_id: usize) -> Self {
        Named(local_id, 0)
    }
}

impl Into<Place> for Named {
    fn into(self) -> Place {
        Place::Named(self)
    }
}

impl Write for Named {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(
            writer,
            "%{}{}",
            function.get_local(self.0).unwrap().get_name().as_ref(),
            self.1
        )
    }
}
