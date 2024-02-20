use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::cfg::PlaceValue;

use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }
}

impl Update for VN {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        let mut constants = HashMap::new();
        let mut vn = HashMap::new();
        let mut next_vn = 0;

        for label in &*cfg {
            let block = cfg.get_block_mut(label);
            let mut delete = vec![];

            for (index, (place, instruction)) in block.get_instructions().iter().enumerate() {
                if let Place::None = place {
                    continue;
                }

                let mut hasher = DefaultHasher::new();
                place.hash(&mut hasher);
                let place_hash = hasher.finish();

                let mut hasher = DefaultHasher::new();
                instruction.hash(&mut hasher, &mut constants);
                let value_hash = hasher.finish();

                if let Some(value_number) = vn.get(&value_hash) {
                    // delete.push(index);
                    vn.insert(place_hash, *value_number);
                } else {
                    vn.insert(place_hash, next_vn);
                    vn.insert(value_hash, next_vn);
                    next_vn += 1;
                }

                if let Instruction::Alias(alias) = instruction {
                    if let PlaceValue::Value(v) = alias.get_place_value() {
                        constants.insert(*place, *v);
                    }
                }
            }

            for index in delete.into_iter().rev() {
                block.delete_instruction(index);
            }
        }

        Ok(())
    }
}
