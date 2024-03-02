use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::*;

mod named;
mod r#static;
mod temp;

pub use named::Named;
pub use r#static::{Static, StaticType};
pub use temp::Temp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Place {
    Named(Named),
    Temp(Temp),
    Static(Static),
    None,
}

impl Place {
    pub fn hash_one(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Into<PlaceValue> for Place {
    fn into(self) -> PlaceValue {
        PlaceValue::Place(self)
    }
}
