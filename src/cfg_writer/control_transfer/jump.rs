use super::*;

impl Write for Jump {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("jump ");
        self.get_label().write(writer, classes, function);
    }
}
