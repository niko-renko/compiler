use super::*;

pub struct Assign;

impl<'cfg> Extract<'cfg, CFG> for Assign {
    fn extract(cfg: &'cfg CFG) -> Result<Self, String> {
        unimplemented!()
    }
}
