use super::*;

pub trait Update {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        function: &Function,
        classes: &Classes,
    ) -> Result<Place, String>;
}
