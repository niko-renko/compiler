use super::*;

impl Write for Named {
    fn write(
        &self,
        writer: &mut Writer,
        _: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        // write!(
        //     writer,
        //     "%{}{}",
        //     function
        //         .get_declaration(self.0)
        //         .unwrap()
        //         .get_name()
        //         .as_ref(),
        //     if self.1 == 0 {
        //         String::from("")
        //     } else {
        //         self.1.to_string()
        //     }
        // )
        Ok(())
    }
}
