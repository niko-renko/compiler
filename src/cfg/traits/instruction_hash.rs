use rand::{thread_rng, Rng};

use super::*;

pub trait InstructionHash {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H);
    fn get_constant(&self, vn: &HashMap<u64, PlaceValue>) -> Option<Value>;

    fn random_hash<H: std::hash::Hasher>(state: &mut H) {
        let random: u64 = thread_rng().gen();
        state.write_u64(random);
    }
}
