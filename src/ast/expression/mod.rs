use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod op;

pub use call::Call;
pub use constant::Constant;
pub use field_read::FieldRead;
pub use local::Local;
pub use new::New;
pub use op::{Op, Operator};

#[derive(Debug)]
pub enum Expression {
    Constant(Constant),
    Local(Local),
    Op(Op),
    Call(Call),
    FieldRead(FieldRead),
    New(New),
}

impl Parse for Expression {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        let first: char = string.chars().next().unwrap();

        if first.is_digit(10) {
            let (next, constant) = Constant::parse(string, false)?;
            return Ok((next, Expression::Constant(constant)));
        }

        if first.is_alphabetic() {
            let (next, local) = Local::parse(string, false)?;
            return Ok((next, Expression::Local(local)));
        }

        if first == '(' {
            let (next, op) = Op::parse(string, false)?;
            return Ok((next, Expression::Op(op)));
        }

        if first == '^' {
            let (next, call) = Call::parse(string, false)?;
            return Ok((next, Expression::Call(call)));
        }

        if first == '&' {
            let (next, field_read) = FieldRead::parse(string, false)?;
            return Ok((next, Expression::FieldRead(field_read)));
        }

        if first == '@' {
            let (next, new) = New::parse(string, false)?;
            return Ok((next, Expression::New(new)));
        }

        Err(String::from("Expected expression"))
    }
}

impl Update for Expression {
    fn update<'cfg>(
        &self,
        cfg: &'cfg mut CFG,
        classes: &Classes,
        function: &Function,
    ) -> Result<Place, String> {
        match self {
            Expression::Constant(constant) => constant.update(cfg, classes, function),
            Expression::Local(local) => local.update(cfg, classes, function),
            Expression::Op(op) => op.update(cfg, classes, function),
            // Expression::Call(call) => call.update(cfg, function, classes),
            // Expression::FieldRead(field_read) => field_read.update(cfg, function, classes),
            // Expression::New(new) => new.update(cfg, function, classes),
            _ => Err(String::from("Not implemented")),
        }
    }
}
