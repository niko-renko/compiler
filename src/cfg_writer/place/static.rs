use super::*;

impl Write for Static {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        _: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // match self.0 {
        //     StaticType::VTable => write!(writer, "@{}", classes.get_vtable_name(self.1)),
        //     StaticType::FieldMap => write!(writer, "@{}", classes.get_fieldmap_name(self.1)),
        // }
        Ok(())
    }
}
