use super::*;

pub struct SSA;

impl SSA {
    pub fn new() -> Self {
        Self
    }
}

impl Update for SSA {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        Ok(())
    }
}
