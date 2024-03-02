use super::*;

impl Write for Phi {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("phi(");
        let mut first = true;
        for (named, label) in self.get_entries() {
            if !first {
                writer.write_code(", ");
            }
            first = false;
            label.write(writer, classes, function);
            writer.write_code(", ");
            named.write(writer, classes, function);
        }
        writer.write_code(")");
    }
}
