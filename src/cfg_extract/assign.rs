use std::collections::HashSet;

use super::*;

pub struct Assign {
    globals: HashSet<Place>,
}

impl Assign {
    pub fn get_globals(&self) -> &HashSet<Place> {
        &self.globals
    }
}

impl<'cfg> Extract<'cfg, CFG> for Assign {
    fn extract(cfg: &'cfg CFG) -> Result<Self, String> {
        let mut globals = HashSet::new();

        for (_, bb) in cfg.get_blocks() {
            let mut var_kill = HashSet::new();
            for (place, instruction) in bb.get_instructions() {
                let places_read = instruction.places_read();

                for place in places_read {
                    if let Place::Named(_) = place {
                        if !var_kill.contains(&place) {
                            globals.insert(place);
                        }
                    }
                }

                var_kill.insert(place);
            }
        }

        Ok(Assign { globals })
    }
}
