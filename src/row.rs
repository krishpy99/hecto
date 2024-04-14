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

/// The information after the row that is highlighted; may affect the highlighting of the next row.
/// For example, if the row ends with a multiline comment, the next row will be highlighted as a multiline comment.
/// Pass the context to the next row to continue highlighting if the operation affects the next row; otherwise, the default value suffices.
#[derive(Default)]
pub struct HighlightContext {
    pub is_in_multiline_comment: bool,
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

    /// Assuming that the character before `from` is not a backslash.
    #[must_use]
    #[allow(clippy::arithmetic_side_effects)] // Overflow checked by `checked_add`.
    fn forms_character_from(&self, from: usize) -> bool {
        if let Some(c) = self.string.as_str().graphemes(true).nth(from) {
            if c == "'" && from.checked_add(1).is_some() {
                // There are two forms:
                // - '.' (single character)
                // - '\.' (escaped character)
                // where '.' is any character except for a backslash.
                if let Some(c) = self.string.as_str().graphemes(true).nth(from + 1) {
                    return (c != "\\"
                        && from.checked_add(2).is_some()
                        && self.string.as_str().graphemes(true).nth(from + 2) == Some("'"))
                        || (c == "\\"
                            && from.checked_add(3).is_some()
                            && self.string.as_str().graphemes(true).nth(from + 3) == Some("'"));
                }
            }
        }
        false
    }

    /// Returns the length of the keyword that starts at `from`. 0 if there is no keyword.
    /// A sub-row is said to form a keyword if it is a prefix of any of the keywords, followed by a separator.
    /// `keyword_len` is updated to the length of the keyword.
    #[must_use]
    fn forms_keyword_from(
        &self,
        from: usize,
        keywords: &Vec<String>,
        // XXX: Out-parameter `keyword_len` is a workaround for assigning in a condition.
        keyword_len: &mut usize,
    ) -> usize {
        // FIXME: Handle UTF-8 characters.
        #[allow(clippy::string_slice)]
        for keyword in keywords {
            if self
                .string
                .get(from..)
                .map_or(false, |s| s.starts_with(keyword))
            {
                if let Some(next_index) = from.checked_add(keyword.len()) {
                    // The separater is either the end of the row (line) or an actual separator.
                    if next_index == self.len() {
                        *keyword_len = keyword.len();
                        return keyword.len();
                    }
                    if let Some(c) = self.string.as_str().graphemes(true).nth(next_index) {
                        if c.chars().next().map_or(false, Self::is_separator) {
                            *keyword_len = keyword.len();
                            return keyword.len();
                        }
                    }
                }
            }
        }
        *keyword_len = 0;
        0
    }

    /// Returns the length of the data type that starts at `from`. 0 if there is no data type.
    /// A sub-row is said to form a data type if it is a prefix of any of the data types, followed by a separator.
    /// `data_type_len` is updated to the length of the data type.
    #[must_use]
    fn forms_data_type_from(
        &self,
        from: usize,
        data_types: &Vec<String>,
        // XXX: Out-parameter `data_type_len` is a workaround for assigning in a condition.
        data_type_len: &mut usize,
    ) -> usize {
        // FIXME: Handle UTF-8 characters.
        #[allow(clippy::string_slice)]
        for data_type in data_types {
            if self
                .string
                .get(from..)
                .map_or(false, |s| s.starts_with(data_type))
            {
                if let Some(next_index) = from.checked_add(data_type.len()) {
                    // The separater is either the end of the row or a punctuation or whitespace.
                    if next_index == self.len() {
                        *data_type_len = data_type.len();
                        return data_type.len();
                    }
                    if let Some(c) = self.string.as_str().graphemes(true).nth(next_index) {
                        if c.chars().next().map_or(false, Self::is_separator) {
                            *data_type_len = data_type.len();
                            return data_type.len();
                        }
                    }
                }
            }
        }
        *data_type_len = 0;
        0
    }

    #[allow(clippy::arithmetic_side_effects)] // Overflow checked by `checked_add`.
    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        ctx: &HighlightContext,
    ) -> HighlightContext {
        // To avoid highlighting part of an identifier as number, we record whether the number is
        // preceded by a separator.
        let mut prev_is_separator = true;
        // Is in keyword if greater than 0.
        let mut remaining_keyword_len = 0usize;
        // Is in data type if greater than 0.
        let mut remaining_data_type_len = 0usize;
        let mut is_in_comment = false;
        let mut is_in_multiline_comment = ctx.is_in_multiline_comment;
        let mut is_in_character = false;
        let mut is_in_string = false;
        let mut is_escaped = false;
        let mut prev_highlight = highlight::Type::None;
        self.highlight = self
            .string
            .chars()
            .enumerate()
            .map(|(i, c)| {
                prev_highlight = if opts.comments && is_in_comment
                    || (c == '/'
                        && i.checked_add(1).is_some()
                        && self.string.chars().nth(i + 1) == Some('/'))
                {
                    // The rest of the line is a comment; not going to end.
                    is_in_comment = true;
                    highlight::Type::Comment
                } else if opts.multiline_comments && is_in_multiline_comment
                    || (c == '/'
                        && i.checked_add(1).is_some()
                        && self.string.chars().nth(i + 1) == Some('*'))
                {
                    if is_in_multiline_comment
                        && c == '/'
                        && i.checked_sub(1).is_some()
                        && self.string.chars().nth(i - 1) == Some('*')
                    {
                        is_in_multiline_comment = false;
                    } else if !is_in_multiline_comment {
                        is_in_multiline_comment = true;
                    }
                    highlight::Type::MultilineComment
                } else if opts.numbers
                    && !is_in_string
                    && (c.is_ascii_digit()
                        || (c == '.'
                            && i.checked_add(1).is_some()
                            && self
                                .string
                                .chars()
                                .nth(i + 1)
                                .map_or(false, |c| c.is_ascii_digit())))
                    && (prev_is_separator || prev_highlight == highlight::Type::Number)
                {
                    highlight::Type::Number
                } else if opts.characters
                    && (is_in_character || (!is_in_string && self.forms_character_from(i)))
                {
                    if c == '\'' && !is_escaped {
                        is_in_character = !is_in_character;
                    }
                    highlight::Type::Character
                } else if opts.strings && (is_in_string || (prev_is_separator && c == '"')) {
                    if c == '"' && !is_escaped {
                        is_in_string = !is_in_string;
                    }
                    highlight::Type::String
                } else if remaining_keyword_len > 0
                    || prev_is_separator
                        && self.forms_keyword_from(i, &opts.keywords, &mut remaining_keyword_len)
                            > 0
                {
                    remaining_keyword_len = remaining_keyword_len.saturating_sub(1);
                    highlight::Type::Keyword
                } else if remaining_data_type_len > 0
                    || prev_is_separator
                        && self.forms_data_type_from(
                            i,
                            &opts.data_types,
                            &mut remaining_data_type_len,
                        ) > 0
                {
                    remaining_data_type_len = remaining_data_type_len.saturating_sub(1);
                    highlight::Type::DataType
                } else {
                    highlight::Type::None
                };
                is_escaped = c == '\\' && !is_escaped;
                prev_is_separator = !is_in_string && Self::is_separator(c);
                prev_highlight
            })
            .collect();
        HighlightContext {
            is_in_multiline_comment,
        }
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

    fn is_separator(c: char) -> bool {
        // '_' can be part of an identifier.
        (c.is_ascii_punctuation() && c != '_') || c.is_ascii_whitespace()
    }
}
