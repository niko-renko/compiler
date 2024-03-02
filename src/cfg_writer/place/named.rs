use super::*;

impl Write for Named {
    fn write(&self, writer: &mut Writer, _: &Classes, function: &FunctionContext) {
        let local_name = function.get_local_name(self.get_id()).unwrap();
        let version = self.get_version();
        writer.write_code(&format!("%{}{}", local_name, version.to_string()));
    }
}
