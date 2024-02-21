use super::*;

pub trait Used {
    fn used(&self) -> Vec<PlaceValue>;
    fn used_mut(&mut self) -> Vec<&mut PlaceValue>;
}
