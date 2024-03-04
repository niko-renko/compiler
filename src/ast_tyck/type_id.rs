use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::*;

pub struct TypeId(usize);

impl TypeId {
    pub fn from(ty: &Type) -> Self {
        let mut hasher = DefaultHasher::new();
        ty.hash(&mut hasher);
        Self(hasher.finish() as usize)
    }
}
