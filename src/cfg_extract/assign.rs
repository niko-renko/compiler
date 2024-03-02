use std::collections::{HashMap, HashSet};

use super::*;

pub struct AssignContext;

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

impl<'cfg> Extract<'cfg, CFG, AssignContext> for Assign {
    fn extract(cfg: &'cfg CFG, _: Option<AssignContext>) -> Result<Self, String> {
        let mut globals = HashSet::new();
        let mut assigned = HashMap::new();

        for label in cfg {
            let block = cfg.get_block(label);
            let mut var_kill = HashSet::new();
            let mut write_read = vec![];

            for (write, instruction) in block.get_instructions() {
                let mut read = vec![];

                for used in instruction.used() {
                    if let PlaceValue::Place(place) = used {
                        read.push(place);
                    }
                }

                write_read.push((write, read));
            }

            for (write, read) in write_read {
                for place in read {
                    if let Place::Named(named) = place {
                        let id = named.get_id();
                        if !var_kill.contains(&id) {
                            globals.insert(id);
                        }
                    }
                }

                if let Place::Named(named) = write {
                    let id = named.get_id();
                    var_kill.insert(id);
                    assigned.entry(id).or_insert(vec![]).push(label);
                }
            }
        }

        Ok(Assign { globals, assigned })
    }
}
