use super::*;

pub struct Branch {
    condition: Place,
    t: Label,
    f: Label,
}

impl Branch {
    pub fn from(condition: Place, t: Label, f: Label) -> Self {
        Branch { condition, t, f }
    }

    pub fn get_true(&self) -> Label {
        self.t
    }

    pub fn get_false(&self) -> Label {
        self.f
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
        self.t.write(writer, classes, function)?;
        write!(writer, " else ")?;
        self.f.write(writer, classes, function)
    }
}
