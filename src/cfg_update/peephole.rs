use super::*;

pub struct Peephole;

impl Peephole {
    pub fn new() -> Self {
        Self
    }
}

impl Update for Peephole {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        unimplemented!()
    }
}
