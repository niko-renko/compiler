use super::*;

#[derive(Clone, Copy)]
pub struct Branch {
    condition: Place,
    true_label: Label,
    false_label: Label,
}

impl Branch {
    pub fn from(condition: Place, true_label: Label, false_label: Label) -> Self {
        Branch {
            condition,
            true_label,
            false_label,
        }
    }

    pub fn get_true(&self) -> Label {
        self.true_label
    }

    pub fn get_false(&self) -> Label {
        self.false_label
    }
}

impl Into<ControlTransfer> for Branch {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Branch(self)
    }
}

impl Write for Branch {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "if ")?;
        self.condition.write(writer, classes, function)?;
        write!(writer, " then ")?;
        self.true_label.write(writer, classes, function)?;
        write!(writer, " else ")?;
        self.false_label.write(writer, classes, function)
    }
}
