use super::*;

impl Write for Value {
    fn write(&self, writer: &mut Writer, _: &Classes, _: &FunctionContext) {
        writer.write_code(&self.get_value().to_string());
    }
}
