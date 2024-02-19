use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaticType {
    VTable,
    FieldMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Static(StaticType, usize);

impl Static {
    pub fn vtable(id: usize) -> Self {
        Self(StaticType::VTable, id)
    }

    pub fn fieldmap(id: usize) -> Self {
        Self(StaticType::FieldMap, id)
    }
}

impl Into<Place> for Static {
    fn into(self) -> Place {
        Place::Static(self)
    }
}

impl Write for Static {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        match self.0 {
            StaticType::VTable => write!(writer, "@{}", classes.get_vtable_name(self.1)),
            StaticType::FieldMap => write!(writer, "@{}", classes.get_fieldmap_name(self.1)),
        }
    }
}
