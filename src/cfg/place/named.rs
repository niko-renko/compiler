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
        Place::Local(self)
    }
}
