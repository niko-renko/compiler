use super::*;

impl Write for PlaceValue {
    fn write(&self, writer: &mut Writer, classes: &Classes, function: &FunctionContext) {
        match self {
            PlaceValue::Place(pv) => pv.write(writer, classes, function),
            PlaceValue::Value(pv) => pv.write(writer, classes, function),
        }
    }
}
