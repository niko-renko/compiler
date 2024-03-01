use super::*;

pub struct Phi(Vec<(PlaceValue, Label)>);

impl Phi {
    pub fn new() -> Self {
        Phi(vec![])
    }

    pub fn get_entries_mut(&mut self) -> Vec<(&mut Named, &mut Label)> {
        let mut entries = vec![];
        for (named, label) in &mut self.0 {
            if let PlaceValue::Place(Place::Named(named)) = named {
                entries.push((named, label));
            }
        }
        entries
    }
}

impl Phi {
    pub fn has_label(&self, label: Label) -> bool {
        self.0.iter().find(|(_, l)| *l == label).is_some()
    }

    pub fn add_entry(&mut self, named: Named, label: Label) {
        let place: Place = named.into();
        self.0.push((place.into(), label));
    }
}

impl Into<Instruction> for Phi {
    fn into(self) -> Instruction {
        Instruction::Phi(self)
    }
}

impl Used for Phi {
    fn used(&self) -> Vec<PlaceValue> {
        self.0.iter().map(|(pv, _)| *pv).collect()
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        self.0.iter_mut().map(|(named, _)| named).collect()
    }
}

impl InstructionHash for Phi {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}

impl Write for Phi {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &FunctionContext,
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
