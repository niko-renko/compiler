use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(usize);

impl Label {
    pub fn from(id: usize) -> Self {
        Label(id)
    }

    pub fn get_id(&self) -> usize {
        self.0
    }
}

impl Write for Label {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        function: &FunctionContext,
    ) -> Result<(), std::io::Error> {
        if self.get_id() == 0 {
            write!(writer, "{}", function.get_name())
        } else {
            write!(writer, "{}_{}", function.get_name(), self.get_id())
        }
    }
}
