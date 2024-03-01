use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceValue {
    Place(Place),
    Value(Value),
}
