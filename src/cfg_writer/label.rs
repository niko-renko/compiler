use super::*;

impl Write for Label {
    fn write(&self, writer: &mut Writer, _: &Classes, function: &FunctionContext) {
        let name = Writer::get_label_name(
            function.get_class().map(|c| c.get_name().as_ref()),
            function.get_function().get_name().as_ref(),
        );

        if self.get_id() == 0 {
            writer.write_code(&name);
        } else {
            writer.write_code(&format!("{}_{}", name, self.get_id()));
        }
    }
}
