//! Char extention for Parser
//!
pub trait AsChar {
    /// makes a char from self
    fn as_char(self) -> char;

    /// tests that self is an alphabetic character
    ///
    /// warning: for `&str` it recognizes alphabetic
    /// characters outside of the 52 ASCII letters
    fn is_alpha(self) -> bool;

    /// tests that self is an alphabetic character
    /// or a decimal digit
    fn is_alphanum(self) -> bool;
    /// tests that self is a decimal digit
    fn is_dec_digit(self) -> bool;
    /// tests that self is an hex digit
    fn is_hex_digit(self) -> bool;
    /// tests that self is an octal digit
    fn is_oct_digit(self) -> bool;
    /// gets the len in bytes for self
    fn len(self) -> usize;
    /// Apply function to char
    fn is_a<F>(self, x: F) -> bool
    where
        F: Fn(&char) -> bool;
}

impl AsChar for char {
    #[inline]
    fn as_char(self) -> char {
        self
    }
    #[inline]
    fn is_alpha(self) -> bool {
        self.is_ascii_alphabetic() || self == '_'
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_dec_digit()
    }
    #[inline]
    fn is_dec_digit(self) -> bool {
        self.is_ascii_digit()
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        self.is_ascii_hexdigit()
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        self.is_digit(8)
    }
    #[inline]
    fn len(self) -> usize {
        self.len_utf8()
    }
    #[inline]
    fn is_a<F>(self, x: F) -> bool
    where
        F: Fn(&char) -> bool,
    {
        x(&self)
    }
}
