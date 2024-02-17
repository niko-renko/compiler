use super::*;

#[derive(Clone, Copy)]
pub enum StaticType {
    VTable,
    FieldMap,
}

#[derive(Clone, Copy)]
pub struct Static(StaticType, usize);

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
