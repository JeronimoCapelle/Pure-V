//! Zero-copy character cursor over UTF-8 assembly source text.

/// Lightweight cursor tracking byte position in `&str`.
#[derive(Debug, Clone, Copy)]
pub(super) struct Cursor<'a> {
    source: &'a str,
    pos: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new [`Cursor`] at byte index `0`.
    pub(super) const fn new(source: &'a str) -> Self {
        Self { source, pos: 0 }
    }

    /// Returns the current byte offset.
    #[inline]
    pub(super) const fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the unread tail of the source string.
    #[inline]
    pub(super) fn rest(&self) -> &'a str {
        &self.source[self.pos..]
    }

    /// Peeks at the next character without consuming it.
    #[inline]
    pub(super) fn first(&self) -> Option<char> {
        self.rest().chars().next()
    }

    /// Peeks at the second unread character without consuming it.
    #[inline]
    pub(super) fn second(&self) -> Option<char> {
        let mut chars = self.rest().chars();
        chars.next()?;
        chars.next()
    }

    /// Consumes and returns the next character.
    pub(super) fn bump(&mut self) -> Option<char> {
        let ch = self.first()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    /// Advances while `pred` returns `true`, stopping before the first
    /// non-matching character.
    pub(super) fn eat_while<F>(&mut self, mut pred: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.first() {
            if pred(ch) {
                self.bump();
            } else {
                break;
            }
        }
    }

    /// Skips horizontal whitespace (`' '`, `'\t'`, `'\r'`) and `#` or `//` line
    /// comments.
    pub(super) fn skip_trivia(&mut self) {
        while let Some(ch) = self.first() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.bump();
                },
                '#' => {
                    self.eat_while(|c| c != '\n');
                },
                '/' if self.second() == Some('/') => {
                    self.eat_while(|c| c != '\n');
                },
                _ => break,
            }
        }
    }

    /// Slices the source text between byte coordinates `[start, end)`.
    pub(super) fn slice(&self, start: usize, end: usize) -> &'a str {
        &self.source[start..end]
    }
}
