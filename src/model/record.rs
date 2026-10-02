#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub value: String,
}

impl Record {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
