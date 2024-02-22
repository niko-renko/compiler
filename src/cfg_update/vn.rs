use std::{collections::HashMap, hash::Hash};

use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }
}

impl Update for VN {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        // hash: instruction | place
        let mut vns: HashMap<Label, HashMap<u64, PlaceValue>> = HashMap::new();

        for label in &*cfg {
            let edges_in = cfg.get_edges_in(label);

            let mut vn = if edges_in.len() == 1 {
                vns.get_mut(&edges_in[0]).unwrap().clone()
            } else {
                HashMap::new()
            };

            let block = cfg.get_block_mut(label);
            let mut delete = vec![];

            for (index, (place, instruction)) in block.get_instructions_mut().iter_mut().enumerate()
            {
                if let Place::None = place {
                    continue;
                }

                for used in instruction.used_mut() {
                    if let PlaceValue::Place(place) = used {
                        let place_hash = place.hash_one();
                        if let Some(pv) = vn.get(&place_hash) {
                            *used = *pv;
                        }
                    }
                }

                let place_hash = place.hash_one();

                if let Some(constant) = instruction.get_constant(&vn) {
                    delete.push(index);
                    vn.insert(place_hash, constant.into());
                    continue;
                }

                let instruction_hash = instruction.hash_one();

                if let Some(_) = vn.get(&instruction_hash) {
                    // println!("HERE");
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

            for used in block.get_end_mut().used_mut() {
                if let PlaceValue::Place(place) = used {
                    let place_hash = place.hash_one();
                    if let Some(pv) = vn.get(&place_hash) {
                        *used = *pv;
                    }
                }
            }

            vns.insert(label, vn);
        }

        Ok(())
    }
}
