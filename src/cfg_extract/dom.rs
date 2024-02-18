use std::collections::{HashMap, HashSet};

use super::*;

#[derive(Debug)]
pub struct Dom {
    dom: HashMap<Label, HashSet<Label>>,
    idom: HashMap<Label, Label>,
    df: HashMap<Label, HashSet<Label>>,
}

impl Dom {
    pub fn get_df(&self) -> &HashMap<Label, HashSet<Label>> {
        &self.df
    }
}

impl Dom {
    fn compute_dom(cfg: &CFG) -> HashMap<Label, HashSet<Label>> {
        let mut dom = HashMap::new();

        for label in cfg {
            let mut set = HashSet::new();
            set.insert(label);
            dom.insert(label, set);
        }

        let mut changed = true;

        while changed {
            changed = false;

            for label in cfg {
                let mut new_dom = cfg
                    .get_edges_in(label)
                    .iter()
                    .map(|pred| dom.get(pred).unwrap())
                    .fold(None, |acc, set| match acc {
                        None => Some(set.clone()),
                        Some(acc) => Some(&acc & set),
                    })
                    .unwrap_or_default();

                new_dom.insert(label);

                if new_dom != *dom.get(&label).unwrap() {
                    changed = true;
                    dom.insert(label, new_dom);
                }
            }
        }

        dom
    }

    fn compute_idom(dom: &HashMap<Label, HashSet<Label>>) -> HashMap<Label, Label> {
        let mut idom = HashMap::new();

        for (label, this_dom) in dom {
            let mut highest = 0;

            for dom_label in this_dom {
                if dom_label == label {
                    continue;
                }

                let len = dom.get(dom_label).unwrap().len();
                if len > highest {
                    idom.insert(*label, *dom_label);
                    highest = len;
                }
            }
        }

        idom
    }

    fn compute_df(cfg: &CFG, idom: &HashMap<Label, Label>) -> HashMap<Label, HashSet<Label>> {
        let mut df = HashMap::new();

        for label in cfg {
            df.insert(label, HashSet::new());
        }

        for label in cfg {
            let preds = cfg.get_edges_in(label);
            if preds.len() > 1 {
                for pred in preds {
                    let mut runner = pred;
                    while runner != *idom.get(&label).unwrap() {
                        df.get_mut(&runner).unwrap().insert(label);
                        runner = *idom.get(&runner).unwrap();
                    }
                }
            }
        }

        df
    }
}

impl<'cfg> Extract<'cfg, CFG> for Dom {
    fn extract(from: &'cfg CFG) -> Result<Self, String> {
        let dom = Dom::compute_dom(from);
        let idom = Dom::compute_idom(&dom);
        let df = Dom::compute_df(from, &idom);

        Ok(Dom { dom, idom, df })
    }
}
