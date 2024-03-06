use super::*;

mod call;
mod constant;
mod field_read;
mod local;
mod new;
mod null;
mod op;

pub use call::Call;
pub use constant::Constant;
pub use field_read::FieldRead;
pub use local::Local;
pub use new::New;
pub use null::Null;
pub use op::{Op, Operator};

pub enum Expression {
    Constant(Constant),
    Local(Local),
    Op(Op),
    Call(Call),
    FieldRead(FieldRead),
    New(New),
    Null(Null),
}

impl Parse for Expression {
    fn try_parse(string: &str) -> Result<(&str, Self), String> {
        if string.starts_with("null") {
            let (next, null) = Null::parse(string, false)?;
            return Ok((next, Expression::Null(null)));
        }

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
