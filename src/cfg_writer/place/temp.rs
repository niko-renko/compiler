use super::*;

impl Write for Temp {
    fn write(&self, writer: &mut Writer, _: &Classes, _: &FunctionContext) {
        writer.write_code("%");
        writer.write_code(&self.get_id().to_string());
    }
}
