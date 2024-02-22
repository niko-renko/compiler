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
        for label in &*cfg {
            let block = cfg.get_block_mut(label);
            let mut delete = vec![];

            // hash: instruction | place
            let mut vn: HashMap<u64, PlaceValue> = HashMap::new();

            for (index, (place, instruction)) in block.get_instructions_mut().iter_mut().enumerate()
            {
                if let Place::None = place {
                    continue;
                }

                for used in instruction.used_mut() {
                    if let PlaceValue::Place(place) = used {
                        let mut hasher = DefaultHasher::new();
                        place.hash(&mut hasher);
                        let place_hash = hasher.finish();

                        if let Some(value) = vn.get(&place_hash) {
                            *used = value.clone();
                        }
                    }
                }

                let mut hasher = DefaultHasher::new();
                place.hash(&mut hasher);
                let place_hash = hasher.finish();

                if let Some(constant) = instruction.get_constant(&vn) {
                    delete.push(index);
                    vn.insert(place_hash, constant.into());
                    continue;
                }

                let mut hasher = DefaultHasher::new();
                instruction.hash(&mut hasher);
                let instruction_hash = hasher.finish();

                if let Some(_) = vn.get(&instruction_hash) {
                    println!("HERE");
                    delete.push(index);
                    vn.insert(place_hash, (*place).into());
                } else {
                    vn.insert(place_hash, (*place).into());
                    vn.insert(instruction_hash, (*place).into());
                }
            }

            for index in delete.into_iter().rev() {
                block.delete_instruction(index);
            }
        }

        Ok(())
    }
}
