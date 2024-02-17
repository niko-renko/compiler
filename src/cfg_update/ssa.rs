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
        for place in assign.get_globals() {
            dbg!(place);
        }
        Ok(())
    }
}
