use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlaceValue {
    Place(Place),
    Value(Value),
}

impl Write for PlaceValue {
    fn write<T: std::io::Write>(
        &self,
        writer: &mut T,
        classes: &Classes,
        function: &Function,
    ) -> Result<(), std::io::Error> {
        match self {
            PlaceValue::Place(pv) => pv.write(writer, classes, function),
            PlaceValue::Value(pv) => pv.write(writer, classes, function),
        }
    }
}
