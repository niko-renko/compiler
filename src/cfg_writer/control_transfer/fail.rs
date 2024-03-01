use super::*;

impl Write for FailReason {
    fn write(
        &self,
        writer: &mut Writer,
        _: &Classes,
        _: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // match self {
        //     FailReason::NotAPointer => write!(writer, "NotAPointer"),
        //     FailReason::NotANumber => write!(writer, "NotANumber"),
        //     FailReason::NoSuchField => write!(writer, "NoSuchField"),
        //     FailReason::NoSuchMethod => write!(writer, "NoSuchMethod"),
        // }
    }
}

impl Write for Fail {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        Ok(())
        // write!(writer, "fail ")?;
        // self.0.write(writer, classes, function)
    }
}
