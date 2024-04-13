use crate::highlight;
use crate::HighlightingOptions;
use core::cmp;

use termion::color;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    string: String,
    highlight: Vec<highlight::Type>,
    len: usize,
}

impl From<&str> for Row {
    fn from(s: &str) -> Self {
        let mut row = Self {
            string: String::from(s),
            highlight: Vec::new(),
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
        let mut curr_highlight = &highlight::Type::None;
        #[allow(clippy::arithmetic_side_effects)]
        for (index, grapheme) in self
            .string
            .as_str()
            .graphemes(true)
            .enumerate()
            .skip(start /* the ones to the left of the screen */)
            .take(end - start /* the visible portion of the row */)
        {
            // A tab is converted to a single space.
            if let Some(c) = grapheme.chars().next() {
                // NOTE: In case some internal error occurs, we want to keep from crashing.
                let highlight_type = self.highlight.get(index).unwrap_or(&highlight::Type::None);
                // Insert a new color sequence only if the color has changed.
                if highlight_type != curr_highlight {
                    curr_highlight = highlight_type;
                    let start_highlight = format!("{}", color::Fg(highlight_type.as_color()));
                    result.push_str(&start_highlight);
                }
                // NOTE: If converting to multiple spaces, special care would be needed to
                // maintain the cursor position, as well as leaving it as it is.
                result.push(if c == '\t' { ' ' } else { c });
            }
        }
        let end_highlight = format!("{}", color::Fg(color::Reset));
        result.push_str(&end_highlight);
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
        self.len = self.string.as_str().graphemes(true).count();
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
        } else {
            // Splits the string into two half, inserts the character after the
            // first part, and then append the second part.
            let mut result: String = self.string.as_str().graphemes(true).take(at).collect();
            let reminder: String = self.string.as_str().graphemes(true).skip(at).collect();
            result.push(c);
            result.push_str(&reminder);
            self.string = result;
        }
        self.update_len();
    }

    #[allow(clippy::arithmetic_side_effects)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }
        let mut result: String = self.string.as_str().graphemes(true).take(at).collect();
        let remainder: String = self.string.as_str().graphemes(true).skip(at + 1).collect();
        result.push_str(&remainder);
        self.string = result;
        self.update_len();
    }

    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.update_len();
    }

    /// Truncates the current row up until a given index, and returns another row with
    /// everything behind that index.
    #[must_use]
    pub fn split(&mut self, at: usize) -> Self {
        let beginning: String = self.string.as_str().graphemes(true).take(at).collect();
        let remainder: String = self.string.as_str().graphemes(true).skip(at).collect();
        self.string = beginning;
        self.update_len();
        Self::from(&*remainder)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    /// Finds the index of the first occurrence of a query string after a given index.
    /// An empty query string will return `None`.
    #[must_use]
    pub fn find_after(&self, query: &str, after: usize) -> Option<usize> {
        if after >= self.len() || query.is_empty() {
            return None;
        }
        let substring: String = self.string.as_str().graphemes(true).skip(after).collect();
        let match_byte_index = substring.find(query);
        if let Some(match_byte_index) = match_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring.as_str().grapheme_indices(true).enumerate()
            {
                if byte_index == match_byte_index {
                    #[allow(clippy::arithmetic_side_effects)]
                    return Some(after + grapheme_index);
                }
            }
        }
        None
    }

    /// Finds the index of the last occurrence of a query string before a given index. `before` is
    /// excluded from the search. An empty query string will return `None`.
    #[must_use]
    pub fn rfind_before(&self, query: &str, before: usize) -> Option<usize> {
        if before == 0 || query.is_empty() {
            return None;
        }
        // NOTE: Since a before exceeding the length of the row doesn't affect the result,
        // we permit it.
        let substring: String = self.string.as_str().graphemes(true).take(before).collect();
        let match_byte_index = substring.rfind(query);
        if let Some(match_byte_index) = match_byte_index {
            for (grapheme_index, (byte_index, _)) in
                substring.as_str().grapheme_indices(true).enumerate()
            {
                if byte_index == match_byte_index {
                    return Some(grapheme_index);
                }
            }
        }
        None
    }

    pub fn highlight(&mut self, opts: HighlightingOptions) {
        // To avoid highlighting part of an identifier as number, we record whether the number is
        // preceded by a separator.
        let mut prev_is_separator = true;
        let mut prev_highlight = highlight::Type::None;
        self.highlight = self
            .string
            .chars()
            .map(|c| {
                prev_highlight = if opts.numbers()
                    && (c.is_ascii_digit() || c == '.')
                    && (prev_is_separator || prev_highlight == highlight::Type::Number)
                {
                    highlight::Type::Number
                } else {
                    highlight::Type::None
                };
                prev_is_separator = c.is_ascii_punctuation() || c.is_ascii_whitespace();
                prev_highlight
            })
            .collect();
    }

    /// Highlights all occurrences of a query string in the row with other words untouched.
    pub fn highlight_query(&mut self, query: &str) {
        // Find the index of all occurrences of the query string.
        let mut matches = Vec::new();
        let mut start = 0;
        while let Some(index) = self.find_after(query, start) {
            matches.push(index);
            if let Some(next_index) = index.checked_add(query.graphemes(true).count()) {
                start = next_index;
            } else {
                break;
            }
        }

        // Highlight the matches.
        while let Some(index) = matches.pop() {
            for i in index..index.saturating_add(query.graphemes(true).count()) {
                if let Some(highlight) = self.highlight.get_mut(i) {
                    *highlight = highlight::Type::Search;
                }
            }
        }
    }
}
