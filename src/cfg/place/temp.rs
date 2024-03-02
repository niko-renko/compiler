use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Temp(usize);

impl Temp {
    pub fn from(temp: usize) -> Self {
        Temp(temp)
    }

    pub fn get_id(&self) -> usize {
        self.0
    }
}

impl Into<Place> for Temp {
    fn into(self) -> Place {
        Place::Temp(self)
    }
}
