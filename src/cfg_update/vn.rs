use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::cfg::Value;

use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }

    fn update_places_used(
        places_used: Vec<&mut PlaceValue>,
        constants: &HashMap<Place, Value>,
        vn: &HashMap<u64, usize>,
        name: &HashMap<usize, Place>,
    ) {
        for place_used in places_used {
            let place = match place_used {
                PlaceValue::Place(place) => place,
                _ => continue,
            };

            if let Some(value) = constants.get(place) {
                *place_used = (*value).into();
                continue;
            }

            let mut hasher = DefaultHasher::new();
            place.hash(&mut hasher);
            let place_hash = hasher.finish();

            if let Some(value_number) = vn.get(&place_hash) {
                *place_used = name.get(value_number).unwrap().clone().into();
            }
        }
    }
}

impl Update for VN {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        let mut vn = HashMap::new();
        let mut next_vn: usize = 0;
        let mut constants = HashMap::new();
        let mut name = HashMap::new();

        for label in &*cfg {
            let block = cfg.get_block_mut(label);
            let mut delete = vec![];

            for (index, (place, instruction)) in block.get_instructions().iter().enumerate() {
                if let Place::None = place {
                    continue;
                }

                if let Some(value) = instruction.get_constant(&mut constants) {
                    constants.insert(*place, value);
                    delete.push(index);
                    continue;
                }

                let mut hasher = DefaultHasher::new();
                place.hash(&mut hasher);
                let place_hash = hasher.finish();

                let mut hasher = DefaultHasher::new();
                instruction.hash(&mut hasher);
                let value_hash = hasher.finish();

                if let Some(value_number) = vn.get(&value_hash) {
                    delete.push(index);
                    vn.insert(place_hash, *value_number);
                } else {
                    vn.insert(place_hash, next_vn);
                    vn.insert(value_hash, next_vn);
                    name.insert(next_vn, *place);
                    next_vn += 1;
                }
            }

            for index in delete.into_iter().rev() {
                block.delete_instruction(index);
            }
        }

        // dbg!(&constants);
        // go through all critical values read
        // and if there's a value number for the place, replace the cannonical place
        // or by a constant value if it was a constant

        for label in &*cfg {
            let block = cfg.get_block_mut(label);

            let places_used: Vec<&mut PlaceValue> = block
                .get_instructions_mut()
                .iter_mut()
                .flat_map(|(_, instruction)| instruction.used_mut())
                .filter(|used| matches!(used, PlaceValue::Place(_)))
                .collect();

            Self::update_places_used(places_used, &constants, &vn, &name);
            Self::update_places_used(block.get_end_mut().used_mut(), &constants, &vn, &name);
        }

        Ok(())
    }
}
