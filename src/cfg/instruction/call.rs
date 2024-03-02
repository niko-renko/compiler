use super::*;

pub struct Call {
    code: PlaceValue,
    object: PlaceValue,
    args: Vec<PlaceValue>,
}

impl Call {
    pub fn from(code: PlaceValue, object: PlaceValue, args: Vec<PlaceValue>) -> Self {
        Self { code, object, args }
    }

    pub fn get_code(&self) -> &PlaceValue {
        &self.code
    }

    pub fn get_object(&self) -> &PlaceValue {
        &self.object
    }

    pub fn get_args(&self) -> &Vec<PlaceValue> {
        &self.args
    }
}

impl Into<Instruction> for Call {
    fn into(self) -> Instruction {
        Instruction::Call(self)
    }
}

impl Used for Call {
    fn used(&self) -> Vec<PlaceValue> {
        let mut used = vec![self.code, self.object];
        used.extend(self.args.clone());
        used
    }

    fn used_mut(&mut self) -> Vec<&mut PlaceValue> {
        let mut used = vec![&mut self.code, &mut self.object];
        used.extend(self.args.iter_mut());
        used
    }
}

impl InstructionHash for Call {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Self::random_hash(state);
    }

    fn get_constant(&self, _: &HashMap<u64, PlaceValue>) -> Option<Value> {
        None
    }
}
