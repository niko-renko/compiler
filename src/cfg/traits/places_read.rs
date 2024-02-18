use super::*;

pub trait PlacesRead {
    fn places_read(&self) -> Vec<Place>;
    fn places_read_mut(&mut self) -> Vec<&mut Place>;
}
