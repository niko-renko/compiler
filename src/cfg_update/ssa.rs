use std::collections::HashSet;

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
            let mut blocks_assigned = assign.get_assigned().get(global).unwrap().clone();
            let mut processed = HashSet::new();

            while !blocks_assigned.is_empty() {
                let block_assigned = blocks_assigned.pop().unwrap();
                let empty = HashSet::new();
                let blocks_read = dom.get_df().get(&block_assigned).unwrap_or(&empty);
                // dbg!(block_assigned, blocks_read);

                for block_read in blocks_read {
                    let block = cfg.get_block(*block_read);

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

        // Do BFS order named numbering

        Ok(())
    }
}
