use super::*;

pub struct Phi(Vec<(Named, Label)>);

impl Phi {
    pub fn from(vec: Vec<(Named, Label)>) -> Self {
        Self(vec)
    }
}

impl Phi {
    pub fn has_label(&self, label: Label) -> bool {
        self.0.iter().find(|(_, l)| *l == label).is_some()
    }

    pub fn add_entry(&mut self, named: Named, label: Label) {
        self.0.push((named, label));
    }
}

impl Write for Phi {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "phi(")?;
        let mut first = true;
        for (named, label) in &self.0 {
            if !first {
                write!(writer, ", ")?;
            }
            first = false;
            label.write(writer, classes, function)?;
            write!(writer, ", ")?;
            named.write(writer, classes, function)?;
        }
        write!(writer, ")")
    }
}
