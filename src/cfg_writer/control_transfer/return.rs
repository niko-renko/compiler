use super::*;

impl Write for Return {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("ret ");
        self.get_place_value().write(writer, classes, function);
    }
}
