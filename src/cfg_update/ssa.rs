use std::collections::{HashMap, HashSet};

use crate::cfg::Instruction;

use super::*;

pub struct SSA;

impl SSA {
    pub fn new() -> Self {
        Self
    }
}

impl Update for SSA {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        let assign = Assign::extract(cfg)?;
        let dom = crate::cfg_extract::Dom::extract(cfg)?;

        for global in assign.get_globals() {
            let empty = vec![];
            let mut blocks_assigned = assign.get_assigned().get(global).unwrap_or(&empty).clone();
            let mut processed = HashSet::new();

            while !blocks_assigned.is_empty() {
                let block_assigned = blocks_assigned.pop().unwrap();
                let empty = HashSet::new();
                let blocks_read = dom.get_df().get(&block_assigned).unwrap_or(&empty);

                for block_read in blocks_read {
                    let block = cfg.get_block_mut(*block_read);

                    if block.has_phi(*global, block_assigned) {
                        continue;
                    }

                    block.add_phi(*global, block_assigned);

                    if !processed.contains(block_read) {
                        processed.insert(*block_read);
                        blocks_assigned.push(*block_read);
                    }
                }
            }
        }

        let mut last_version = HashMap::new();
        let mut last_in_blocks: HashMap<Label, HashMap<usize, usize>> = HashMap::new();

        for label in &*cfg {
            let block = cfg.get_block_mut(label);
            let mut last_in_block = HashMap::new();

            for (place, instruction) in block.get_instructions_mut() {
                if let Instruction::Phi(phi) = instruction {
                    for (named, label) in phi.get_entries_mut() {
                        let last_in_block = last_in_blocks.entry(*label).or_default();
                        let version = last_in_block.get(&named.get_id()).unwrap_or(&0);
                        named.set_version(*version);
                    }
                } else {
                    for used in instruction.used_mut() {
                        if let PlaceValue::Place(Place::Named(named)) = used {
                            let version = last_version.entry(named.get_id()).or_default();
                            named.set_version(*version);
                        }
                    }
                }

                if let Place::Named(named) = place {
                    let new_version = last_version.get(&named.get_id()).unwrap_or(&0) + 1;
                    last_version.insert(named.get_id(), new_version);
                    last_in_block.insert(named.get_id(), new_version);
                    named.set_version(new_version);
                }
            }

            for used in block.get_end_mut().used_mut() {
                if let PlaceValue::Place(Place::Named(named)) = used {
                    let version = last_version.entry(named.get_id()).or_default();
                    named.set_version(*version);
                }
            }

            last_in_blocks.insert(label, last_in_block);
        }

        Ok(())
    }
}
