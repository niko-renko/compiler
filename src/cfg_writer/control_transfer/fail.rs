use super::*;

impl Write for FailReason {
    fn write(&self, writer: &mut Writer, _: &Classes, _: &FunctionContext) {
        match self {
            FailReason::NotAPointer => writer.write_code("NotAPointer"),
            FailReason::NotANumber => writer.write_code("NotANumber"),
        }
    }
}

impl Write for Fail {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        writer.write_code("fail ");
        self.get_reason().write(writer, classes, function);
    }
}
