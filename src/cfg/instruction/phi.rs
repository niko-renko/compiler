use super::*;

pub struct Phi(Vec<(Named, Label)>);

impl Phi {
    pub fn new() -> Self {
        Phi(vec![])
    }

    pub fn get_entries_mut(&mut self) -> &mut Vec<(Named, Label)> {
        &mut self.0
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

impl Into<Instruction> for Phi {
    fn into(self) -> Instruction {
        Instruction::Phi(self)
    }
}

impl PlacesRead for Phi {
    fn places_read(&self) -> Vec<Place> {
        vec![]
    }

    fn places_read_mut(&mut self) -> Vec<&mut Place> {
        vec![]
    }
}

impl InstructionHash for Phi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, usize>) {
        Self::random_hash(state);
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
