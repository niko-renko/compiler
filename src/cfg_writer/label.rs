use super::*;

impl Write for Label {
    fn write(
        &self,
        writer: &mut Writer,
        _: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // if self.get_id() == 0 {
        //     write!(writer, "{}", function.get_name())
        // } else {
        //     write!(writer, "{}_{}", function.get_name(), self.get_id())
        // }
        Ok(())
    }
}
