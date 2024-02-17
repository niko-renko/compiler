use super::*;

#[derive(Clone, Copy)]
pub struct Temp(usize);

impl Temp {
    pub fn from(temp: usize) -> Self {
        Temp(temp)
    }
}

impl Into<Place> for Temp {
    fn into(self) -> Place {
        Place::Temp(self)
    }
}

impl Write for Temp {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "%{}", self.0)
    }
}
