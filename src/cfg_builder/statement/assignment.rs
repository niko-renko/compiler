use super::*;

impl Build for AssignmentLeft {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        match self {
            AssignmentLeft::Local(local) => local.update(cfg, classes, function),
            AssignmentLeft::Empty => Ok(Place::None),
        }
    }
}

impl Build for Assignment {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let left = self.get_left().update(cfg, classes, function)?;
        let right = self.get_right().update(cfg, classes, function)?;

        if let Place::None = left {
            return Ok(right);
        }

        cfg.add_placed(left, Alias::from(right.into()).into());
        Ok(left)
    }
}
