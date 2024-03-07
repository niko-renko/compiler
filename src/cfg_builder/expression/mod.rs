use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod null;
mod op;

impl Build for Expression {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        types: &Types,
        function: &FunctionContext,
    ) -> Result<Place, String> {
        match self {
            Expression::Constant(e) => e.update(cfg, classes, types, function),
            Expression::Local(e) => e.update(cfg, classes, types, function),
            Expression::Op(e) => e.update(cfg, classes, types, function),
            Expression::Call(e) => e.update(cfg, classes, types, function),
            Expression::FieldRead(e) => e.update(cfg, classes, types, function),
            Expression::New(e) => e.update(cfg, classes, types, function),
            Expression::Null(e) => e.update(cfg, classes, types, function),
        }
    }
}
