use super::*;

#[derive(Clone, Copy)]
pub enum FailReason {
    NotAPointer,
    NotANumber,
    NoSuchField,
    NoSuchMethod,
}

impl Write for FailReason {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        _: &Classes,
        _: &Function,
    ) -> Result<(), std::io::Error> {
        match self {
            FailReason::NotAPointer => write!(writer, "NotAPointer"),
            FailReason::NotANumber => write!(writer, "NotANumber"),
            FailReason::NoSuchField => write!(writer, "NoSuchField"),
            FailReason::NoSuchMethod => write!(writer, "NoSuchMethod"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Fail(FailReason);

impl Fail {
    pub fn from(reason: FailReason) -> Self {
        Fail(reason)
    }
}

impl Into<ControlTransfer> for Fail {
    fn into(self) -> ControlTransfer {
        ControlTransfer::Fail(self)
    }
}

impl Write for Fail {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        write!(writer, "fail ")?;
        self.0.write(writer, classes, function)
    }
}

impl PlacesRead for Fail {
    fn places_read(&self) -> Vec<Place> {
        vec![]
    }
}
