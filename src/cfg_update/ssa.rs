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

        for place in assign.get_globals() {
            let mut work_list: Vec<Label> = assign.get_assigned().get(place).unwrap().clone();

            while !work_list.is_empty() {
                let label = work_list.pop().unwrap();
            }
        }

        Ok(())
    }
}
