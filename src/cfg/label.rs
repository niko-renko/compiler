#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(usize);

impl Label {
    pub fn from(id: usize) -> Self {
        Label(id)
    }

    pub fn get_id(&self) -> usize {
        self.0
    }
}
