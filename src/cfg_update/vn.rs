use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault};

use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }
}

impl Update for VN {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        let hasher: BuildHasherDefault<DefaultHasher> = Default::default();
        let mut vn = HashMap::new();
        let mut next_vn = 0;

        for label in &*cfg {
            let block = cfg.get_block_mut(label);
            let mut delete = vec![];

            for (index, (place, instruction)) in block.get_instructions().iter().enumerate() {
                if let Place::None = place {
                    continue;
                }

                let place_hash = hasher.hash_one(*place);
                let value_hash = hasher.hash_one(instruction);

                if let Some(value_number) = vn.get(&value_hash) {
                    delete.push(index);
                    vn.insert(place_hash, *value_number);
                } else {
                    vn.insert(place_hash, next_vn);
                    vn.insert(value_hash, next_vn);
                    next_vn += 1;
                }
            }

            for index in delete.into_iter().rev() {
                block.delete_instruction(index);
            }
        }

        Ok(())
    }
}
