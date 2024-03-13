use super::*;

impl Build for ast::Op {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        let left = self.get_left().update(cfg, classes, types, function)?;
        let right = self.get_right().update(cfg, classes, types, function)?;
        let operator = self.get_operator();

        if matches!(operator, ast::Operator::Div) {
            let is_zero = cfg
                .add(cfg::Op::from(right.into(), Value::from(0).into(), cfg::Operator::Eq).into());
            cfg.fail_if(is_zero, true, FailReason::NotANumber);
        }

        let cfg_operator = match operator {
            ast::Operator::Add => cfg::Operator::Add,
            ast::Operator::Sub => cfg::Operator::Sub,
            ast::Operator::Mul => cfg::Operator::Mul,
            ast::Operator::Div => cfg::Operator::Div,
            ast::Operator::Eq => cfg::Operator::Eq,
        };

        let result = cfg.add(cfg::Op::from(left.into(), right.into(), cfg_operator).into());
        Ok(result)
    }
}
