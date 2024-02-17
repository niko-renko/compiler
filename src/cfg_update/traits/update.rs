use super::*;

pub trait Update {
    fn update(&self, cfg: &mut CFG) -> Result<(), String>;
}
