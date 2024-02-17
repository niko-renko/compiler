use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Debug)]
pub struct Dom(HashMap<Label, HashSet<Label>>);

impl<'cfg> Extract<'cfg, CFG> for Dom {
    fn extract(from: &'cfg CFG) -> Result<Self, String> {
        let mut dom = HashMap::new();

        for (label, _) in from.get_blocks() {
            let mut set = HashSet::new();
            set.insert(*label);
            dom.insert(*label, set);
        }

        let mut changed = true;

        while changed {
            changed = false;

            for (label, _) in from.get_blocks() {
                let mut new_dom = from
                    .get_preds(*label)
                    .iter()
                    .map(|pred| dom.get(pred).unwrap())
                    .fold(None, |acc, set| match acc {
                        None => Some(set.clone()),
                        Some(acc) => Some(&acc & set),
                    })
                    .unwrap_or_default();

                new_dom.insert(*label);

                if new_dom != *dom.get(label).unwrap() {
                    changed = true;
                    dom.insert(*label, new_dom);
                }
            }
        }

        Ok(Dom(dom))
    }
}
