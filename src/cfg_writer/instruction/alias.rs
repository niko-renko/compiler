use super::*;

impl Write for Alias {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        self.get_place_value().write(writer, classes, function);
    }
}
