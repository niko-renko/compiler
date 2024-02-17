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
