use super::*;

pub trait InstructionHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H, constants: &mut HashMap<Place, usize>);
}
