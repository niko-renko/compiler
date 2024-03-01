use super::*;

mod named;
mod r#static;
mod temp;

impl Write for Place {
    fn write(
        &self,
        writer: &mut Writer,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        match self {
            Place::Named(p) => p.write(writer, classes, function),
            Place::Temp(p) => p.write(writer, classes, function),
            Place::Static(p) => p.write(writer, classes, function),
            Place::None => Ok(()),
        }
    }
}
