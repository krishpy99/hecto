use std::cmp;

use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let mut row = Self {
            string: String::from(s),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    #[must_use]
    pub fn render(&self, start: usize, end: usize) -> String {
        // Get the actual end of such row.
        let end = cmp::min(end, self.string.len());
        // In case that `start` is greater than `end`, we want to return an empty string.
        let start = cmp::min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start /* the ones to the left of the screen */)
            .take(end - start /* the visible portion of the row */)
        {
            // A tab is converted to 2 spaces.
            result.push_str(if grapheme == "\t" { "  " } else { grapheme });
        }
        result
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// To avoid recomputing the length of the row every time we need it.
    fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
}
