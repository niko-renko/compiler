use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaticType {
    VTable,
    FieldMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Static(StaticType, usize);

impl Static {
    pub fn vtable_from(id: usize) -> Self {
        Self(StaticType::VTable, id)
    }

    pub fn fieldmap_from(id: usize) -> Self {
        Self(StaticType::FieldMap, id)
    }
}

impl Into<Place> for Static {
    fn into(self) -> Place {
        Place::Static(self)
    }
}
