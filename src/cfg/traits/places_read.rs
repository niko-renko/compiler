use super::*;

pub trait PlacesRead {
    fn places_read(&self) -> Vec<Place>;
}
