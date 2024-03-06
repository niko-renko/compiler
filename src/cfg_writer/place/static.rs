use super::*;

impl Write for Static {
    fn write(&self, writer: &mut Writer, classes: &Classes, _: &FunctionContext) {
        let static_type = self.get_type();

        let class_id = self.get_id();
        let class_name = classes
            .get_class_name(class_id)
            .map(|name| name.as_ref())
            .unwrap();

        writer.write_code("@");
        match static_type {
            StaticType::VTable => writer.write_code(&Writer::get_vtable_name(class_name)),
            StaticType::FieldMap => writer.write_code(&Writer::get_fieldmap_name(class_name)),
        }
    }
}
