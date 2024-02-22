use std::collections::HashMap;

use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }

    fn replace(used: Vec<&mut PlaceValue>, vn: &HashMap<u64, PlaceValue>) {
        for used in used {
            let canonical = match used {
                PlaceValue::Place(place) => vn.get(&place.hash_one()),
                _ => continue,
            };

            let canonical = match canonical {
                Some(canonical) => canonical,
                None => continue,
            };

            // println!("Replacing {:?} with {:?}", used, canonical);
            *used = *canonical;
        }
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
                Self::replace(instruction.used_mut(), &vn);

                if let Place::None = place {
                    continue;
                }

                let place_hash = place.hash_one();

                if let Some(constant) = instruction.get_constant(&vn) {
                    delete.push(index);
                    vn.insert(place_hash, constant.into());
                    continue;
                }

                let instruction_hash = instruction.hash_one();

                if let Some(canonical) = vn.get(&instruction_hash) {
                    delete.push(index);
                    vn.insert(place_hash, *canonical);
                } else {
                    vn.insert(place_hash, (*place).into());
                    vn.insert(instruction_hash, (*place).into());
                }
            }

            for index in delete.into_iter().rev() {
                block.delete_instruction(index);
            }

            Self::replace(block.get_end_mut().used_mut(), &vn);
            vns.insert(label, vn);
        }

        Ok(())
    }
}
