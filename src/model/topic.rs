use super::Record;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Topic {
    records: Vec<Record>,
}

impl Topic {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub(crate) fn store(&mut self, record: Record) {
        self.records.push(record);
    }
}
