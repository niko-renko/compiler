use super::*;

impl Write for Named {
    fn write(&self, writer: &mut Writer, _: &Classes, function: &FunctionContext) {
        let local_name = function.get_local_declaration(self.get_id()).unwrap();
        let version = self.get_version();

        if version == 0 {
            writer.write_code(&format!("%{}", local_name.get_name().as_ref()));
            return;
        }

        writer.write_code(&format!(
            "%{}{}",
            local_name.get_name().as_ref(),
            version.to_string()
        ));
    }
}
