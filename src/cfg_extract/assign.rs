use std::collections::{HashMap, HashSet};

use super::*;

pub struct Assign {
    globals: HashSet<usize>,
    assigned: HashMap<usize, Vec<Label>>,
}

impl Assign {
    pub fn get_globals(&self) -> &HashSet<usize> {
        &self.globals
    }

    pub fn get_assigned(&self) -> &HashMap<usize, Vec<Label>> {
        &self.assigned
    }
}

impl<'cfg> Extract<'cfg, CFG> for Assign {
    fn extract(cfg: &'cfg CFG) -> Result<Self, String> {
        let mut globals = HashSet::new();
        let mut assigned = HashMap::new();

        for (label, bb) in cfg.get_blocks() {
            let mut var_kill = HashSet::new();
            for (place, instruction) in bb.get_instructions() {
                let places_read = instruction.places_read();

                for place in places_read {
                    if let Place::Named(named) = place {
                        let id = named.get_id();
                        if !var_kill.contains(&id) {
                            globals.insert(id);
                        }
                    }
                }

                if let Place::Named(named) = place {
                    let id = named.get_id();
                    var_kill.insert(id);
                    assigned.entry(id).or_insert(vec![]).push(*label);
                }
            }
        }

        Ok(Assign { globals, assigned })
    }
}
