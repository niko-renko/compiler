use super::*;

pub struct VN;

impl VN {
    pub fn new() -> Self {
        VN
    }
}

impl Update for VN {
    fn update(&self, cfg: &mut CFG) -> Result<(), String> {
        unimplemented!()
    }
}
