use super::*;

impl Build for AssignmentLeft {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        match self {
            AssignmentLeft::Local(local) => local.update(cfg, classes, types, function),
            AssignmentLeft::Empty => Ok(Place::None),
        }
    }
}

impl Build for Assignment {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let left = self.get_left().update(cfg, classes, types, function)?;
        let right = self.get_right().update(cfg, classes, types, function)?;

        if let Place::None = left {
            return Ok(right);
        }

        cfg.add_placed(left, Alias::from(right.into()).into());
        Ok(left)
    }
}
