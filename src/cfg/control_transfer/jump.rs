use super::*;

#[derive(Clone, Copy)]
pub struct Jump(Label);

impl Jump {
    pub fn from(label: Label) -> Self {
        Jump(label)
    }

    pub fn get_label(&self) -> Label {
        self.0
    }
}

impl Into<ControlTransfer> for Jump {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Jump(self)
    }
}

impl PlacesRead for Jump {
    fn places_read(&self) -> Vec<Place> {
        vec![]
    }
}

impl Write for Jump {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "jump ")?;
        self.0.write(writer, classes, function)
    }
}
