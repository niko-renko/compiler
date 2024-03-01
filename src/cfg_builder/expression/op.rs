use super::*;

impl Build for ast::Op {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let left = self.get_left().update(cfg, classes, function)?;
        cfg.fail_if_ptr(left);

        let right = self.get_right().update(cfg, classes, function)?;
        cfg.fail_if_ptr(right);

        match self.get_operator() {
            ast::Operator::Add => {
                let left = cfg.add_const(left, -1);
                Ok(cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Add).into()))
            }
            ast::Operator::Sub => {
                let untagged =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Sub).into());

                Ok(cfg.add_const(untagged, 1))
            }
            ast::Operator::Mul => {
                let left = cfg.to_raw(left);
                let right = cfg.to_raw(right);

                let raw =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Mul).into());

                Ok(cfg.to_int(raw))
            }
            ast::Operator::Div => {
                let is_zero = cfg.add(
                    cfg::Op::from(right.into(), Value::from(0).into(), cfg::Operator::Eq).into(),
                );

                cfg.fail_if(is_zero, true, FailReason::NotANumber);

                let left = cfg.add_const(left, -1);
                let right = cfg.add_const(right, -1);

                let raw =
                    cfg.add(cfg::Op::from(left.into(), right.into(), cfg::Operator::Div).into());

                Ok(cfg.to_int(raw))
            }
        }
    }
}
