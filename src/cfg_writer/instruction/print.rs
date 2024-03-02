use super::*;

impl Write for Print {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("print(");
        self.get_value().write(writer, classes, function);
        writer.write_code(")")
    }
}
