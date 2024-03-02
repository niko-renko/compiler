use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Named(usize, usize);

impl Named {
    pub fn from(local_id: usize) -> Self {
        Named(local_id, 0)
    }

    pub fn get_id(&self) -> usize {
        self.0
    }

    pub fn get_version(&self) -> usize {
        self.1
    }

    pub fn set_version(&mut self, version: usize) {
        self.1 = version;
    }
}

impl Into<Place> for Named {
    fn into(self) -> Place {
        Place::Named(self)
    }
}
