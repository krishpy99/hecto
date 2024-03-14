use crate::Row;

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    /// # Errors
    /// Returns an error if the file can't be read.
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in content.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self { rows })
    }

    #[must_use]
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// Whether the document is empty or no documents have been loaded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.rows.len()
    }
}
