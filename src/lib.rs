//! A library for storing binary data as a string.
//!
//! This library provides a `BinString` type that wraps a `String` and provides
//! conversion methods between binary data and strings.
//!
//! # Safety
//!
//! While storing invalid UTF-8 in a `String` is perfectly safe, attempting to treat
//! the string as valid UTF-8 (for example, by displaying it or using string operations
//! that assume valid UTF-8) may lead to undefined behavior.
//!
//! The methods defined here, and the `Debug` and `Display` implementations, work on raw bytes and are fine with any data.
//!
//! `Debug` shows invalid UTF-8 as `\xNN` escapes, and `Display` writes the bytes as they are.
//!
//! Methods that come from `String` or `str` through `Deref`, like `chars()` or `pop()`, expect valid UTF-8.
//! Only use them when you know the data is valid text.
//!
//! # Examples
//!
//! ```
//! use binstring::BinString;
//!
//! let bin_str = BinString::new("hello");
//! assert_eq!(bin_str.as_str(), "hello");
//!
//! let bytes = vec![104, 101, 108, 108, 111]; // "hello" in bytes
//! let bin_str = BinString::from_bytes(bytes);
//! assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
//! ```

use std::borrow::Borrow;
use std::fmt;
use std::iter::FromIterator;
use std::ops;
use std::ops::{Deref, DerefMut};

/// A type that wraps a `String` and provides conversion methods between binary data and strings.
///
/// # Examples
///
/// ```
/// use binstring::BinString;
///
/// let bin_str = BinString::new("hello");
/// assert_eq!(bin_str.as_str(), "hello");
/// ```
#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BinString(pub String);

impl BinString {
    /// Creates a new `BinString` from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("hello");
    /// assert_eq!(bin_str.as_str(), "hello");
    /// ```
    #[inline]
    pub fn new(s: impl Into<String>) -> Self {
        BinString(s.into())
    }

    /// Creates a new `BinString` from bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bytes = vec![104, 101, 108, 108, 111]; // "hello" in bytes
    /// let bin_str = BinString::from_bytes(bytes);
    /// assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
    /// ```
    #[inline]
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        BinString(unsafe { String::from_utf8_unchecked(bytes.into()) })
    }

    /// Consumes the `BinString` and returns the underlying `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("hello");
    /// let s = bin_str.unwrap();
    /// assert_eq!(s, "hello");
    /// ```
    #[inline]
    pub fn unwrap(self) -> String {
        self.0
    }

    /// Returns a string slice of the entire `BinString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("hello");
    /// assert_eq!(bin_str.as_str(), "hello");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns a byte slice of the entire `BinString`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("hello");
    /// assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
    /// ```
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns the length of the `BinString` in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("hello");
    /// assert_eq!(bin_str.len(), 5);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the `BinString` has a length of zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let bin_str = BinString::new("");
    /// assert!(bin_str.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a new `BinString` containing the concatenation of this string and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s1 = BinString::new("hello");
    /// let s2 = BinString::new(" world");
    /// assert_eq!(s1.concat(&s2).as_str(), "hello world");
    /// ```
    #[inline]
    pub fn concat(&self, other: &BinString) -> BinString {
        let mut bytes = self.as_bytes().to_vec();
        bytes.extend_from_slice(other.as_bytes());
        BinString::from_bytes(bytes)
    }

    /// Returns a new `BinString` containing a slice of this string.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert_eq!(s.slice(1..4).as_str(), "ell");
    /// ```
    pub fn slice<R: ops::RangeBounds<usize>>(&self, range: R) -> BinString {
        let bounds = (range.start_bound().cloned(), range.end_bound().cloned());
        BinString::from_bytes(&self.as_bytes()[bounds])
    }

    /// Returns `true` if this string starts with the given bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert!(s.starts_with(&[104, 101])); // "he"
    /// ```
    #[inline]
    pub fn starts_with(&self, prefix: &[u8]) -> bool {
        self.as_bytes().starts_with(prefix)
    }

    /// Returns `true` if this string ends with the given bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert!(s.ends_with(&[108, 111])); // "lo"
    /// ```
    #[inline]
    pub fn ends_with(&self, suffix: &[u8]) -> bool {
        self.as_bytes().ends_with(suffix)
    }

    /// Returns `true` if this string contains the given bytes.
    ///
    /// An empty needle is always found.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert!(s.contains(&[101, 108])); // "el"
    /// ```
    #[inline]
    pub fn contains(&self, needle: &[u8]) -> bool {
        self.find(needle).is_some()
    }

    /// Returns the index of the first occurrence of the given bytes.
    ///
    /// Returns `None` if the bytes are not found.
    /// An empty needle is found at index 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert_eq!(s.find(&[101, 108]), Some(1)); // "el"
    /// assert_eq!(s.find(&[120]), None); // "x"
    /// ```
    #[inline]
    pub fn find(&self, needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        self.as_bytes()
            .windows(needle.len())
            .position(|window| window == needle)
    }

    /// Returns the index of the last occurrence of the given bytes.
    ///
    /// Returns `None` if the bytes are not found.
    /// An empty needle is found at the end, at index `len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert_eq!(s.rfind(&[108]), Some(3)); // "l"
    /// assert_eq!(s.rfind(&[120]), None); // "x"
    /// ```
    #[inline]
    pub fn rfind(&self, needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(self.len());
        }
        self.as_bytes()
            .windows(needle.len())
            .rposition(|window| window == needle)
    }

    /// Returns a new `BinString` with all occurrences of a byte replaced with another.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("hello");
    /// assert_eq!(s.replace(108, 120).as_bytes(), &[104, 101, 120, 120, 111]); // "hexxo"
    /// ```
    #[inline]
    pub fn replace(&self, from: u8, to: u8) -> BinString {
        let mut bytes = self.as_bytes().to_vec();
        for byte in &mut bytes {
            if *byte == from {
                *byte = to;
            }
        }
        BinString::from_bytes(bytes)
    }

    /// Returns a new `BinString` with leading and trailing whitespace removed.
    ///
    /// For valid UTF-8, this does the same as `str::trim()`.
    ///
    /// For binary data, trimming stops at the first invalid byte on each side, and invalid bytes are always kept.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("  hello  ");
    /// assert_eq!(s.trim().as_str(), "hello");
    ///
    /// let s = BinString::from_bytes(b" \xffhello\xfe ".to_vec());
    /// assert_eq!(s.trim().as_bytes(), b"\xffhello\xfe");
    /// ```
    pub fn trim(&self) -> BinString {
        let bytes = self.as_bytes();
        let mut chunks = bytes.utf8_chunks();
        let Some(first) = chunks.next() else {
            return BinString::default();
        };
        // Everything is valid UTF-8
        if first.invalid().is_empty() {
            return BinString::new(first.valid().trim());
        }
        let start = first.valid().len() - first.valid().trim_start().len();
        let last = chunks.last().unwrap_or(first);
        let end = if last.invalid().is_empty() {
            bytes.len() - last.valid().len() + last.valid().trim_end().len()
        } else {
            bytes.len()
        };
        BinString::from_bytes(&bytes[start..end])
    }

    /// Checks if the `BinString` contains only valid UTF-8 characters.
    ///
    /// This is useful to determine if the `BinString` can be safely used as a Rust string.
    /// If this returns `false`, the `BinString` should be treated as binary data only.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let valid = BinString::new("hello");
    /// assert!(valid.is_valid_utf8());
    ///
    /// let bytes = vec![0xFF, 0xFE, 0xFD];
    /// let invalid = BinString::from_bytes(bytes);
    /// assert!(!invalid.is_valid_utf8());
    /// ```
    #[inline]
    pub fn is_valid_utf8(&self) -> bool {
        std::str::from_utf8(self.as_bytes()).is_ok()
    }
}

/// Same output as `String`'s `Debug`, but invalid UTF-8 shows up as `\xNN` escapes.
struct EscapedBytes<'a>(&'a [u8]);

impl fmt::Debug for EscapedBytes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\"")?;
        for chunk in self.0.utf8_chunks() {
            for c in chunk.valid().chars() {
                // String's Debug doesn't escape single quotes, so neither do we
                if c == '\'' {
                    f.write_str("'")?;
                } else {
                    write!(f, "{}", c.escape_debug())?;
                }
            }
            for byte in chunk.invalid() {
                write!(f, "\\x{byte:02x}")?;
            }
        }
        f.write_str("\"")
    }
}

impl fmt::Debug for BinString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BinString")
            .field(&EscapedBytes(self.as_bytes()))
            .finish()
    }
}

impl fmt::Display for BinString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for BinString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for BinString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<String> for BinString {
    #[inline]
    fn from(s: String) -> Self {
        BinString::new(s)
    }
}

impl From<BinString> for String {
    #[inline]
    fn from(s: BinString) -> Self {
        s.0
    }
}

impl From<&str> for BinString {
    #[inline]
    fn from(s: &str) -> Self {
        BinString::new(s.to_string())
    }
}

impl From<&[u8]> for BinString {
    #[inline]
    fn from(bytes: &[u8]) -> Self {
        BinString::from_bytes(bytes.to_vec())
    }
}

impl From<Vec<u8>> for BinString {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        BinString::from_bytes(bytes)
    }
}

impl From<&Vec<u8>> for BinString {
    #[inline]
    fn from(bytes: &Vec<u8>) -> Self {
        BinString::from_bytes(bytes.to_owned())
    }
}

impl Default for BinString {
    #[inline]
    fn default() -> Self {
        BinString(String::new())
    }
}

impl Deref for BinString {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for BinString {
    #[inline]
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl Borrow<str> for BinString {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Extend<char> for BinString {
    #[inline]
    fn extend<T: IntoIterator<Item = char>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a char> for BinString {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a char>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl Extend<String> for BinString {
    #[inline]
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<'a> Extend<&'a str> for BinString {
    #[inline]
    fn extend<T: IntoIterator<Item = &'a str>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<char> for BinString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        BinString::new(String::from_iter(iter))
    }
}

impl<'a> FromIterator<&'a char> for BinString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        BinString::new(String::from_iter(iter))
    }
}

impl FromIterator<String> for BinString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        BinString::new(String::from_iter(iter))
    }
}

impl<'a> FromIterator<&'a str> for BinString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        BinString::new(String::from_iter(iter))
    }
}

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        let bytes: &[u8] = b"hello";
        for s in [
            BinString::new("hello"),
            BinString::from_bytes(bytes),
            BinString::from("hello"),
            BinString::from(bytes),
            BinString::from(bytes.to_vec()),
        ] {
            assert_eq!(s.as_bytes(), bytes);
            assert_eq!(s.as_str(), "hello");
        }

        let s = BinString::new("hello");
        let borrowed: &str = s.borrow();
        assert_eq!(borrowed, "hello");
        assert_eq!(s.unwrap(), "hello");
    }

    #[test]
    fn test_len() {
        assert_eq!(BinString::new("hello").len(), 5);
        assert!(BinString::new("").is_empty());
        assert!(BinString::default().is_empty());
    }

    #[test]
    fn test_concat() {
        let s = BinString::new("hello").concat(&BinString::new(" world"));
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_slice() {
        let s = BinString::new("hello");
        assert_eq!(s.slice(1..4).as_bytes(), b"ell");
        assert_eq!(s.slice(..=1).as_bytes(), b"he");
        assert_eq!(s.slice(5..).as_bytes(), b"");

        // Slices on bytes, not characters
        let s = BinString::new("\u{1f600}");
        assert_eq!(s.slice(1..3).as_bytes(), b"\x9f\x98");
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds() {
        let _ = BinString::new("hello").slice(..=usize::MAX);
    }

    #[test]
    fn test_search() {
        let s = BinString::new("hello");
        assert!(s.starts_with(b"he"));
        assert!(s.ends_with(b"lo"));
        assert!(s.contains(b"el"));
        assert!(!s.contains(b"hello world"));
        assert_eq!(s.find(b"l"), Some(2));
        assert_eq!(s.rfind(b"l"), Some(3));
        assert_eq!(s.find(b"x"), None);
        assert_eq!(s.rfind(b"x"), None);
        assert_eq!(s.find(b""), Some(0));
        assert_eq!(s.rfind(b""), Some(5));
    }

    #[test]
    fn test_replace() {
        let s = BinString::new("hello").replace(b'l', b'x');
        assert_eq!(s.as_bytes(), b"hexxo");
    }

    #[test]
    fn test_trim() {
        let cases: [(&[u8], &[u8]); 4] = [
            (b"", b""),
            (b"  hello  ", b"hello"),
            (b" a\xff", b"a\xff"),
            // Starts and ends with non-ASCII spaces (U+3000 and U+2003)
            (b"\xe3\x80\x80 a \xff b\xe2\x80\x83\n", b"a \xff b"),
        ];
        for (input, expected) in cases {
            assert_eq!(BinString::from_bytes(input).trim().as_bytes(), expected);
        }
    }

    #[test]
    fn test_is_valid_utf8() {
        assert!(BinString::new("hello").is_valid_utf8());
        assert!(!BinString::from_bytes(b"\xff\xfe").is_valid_utf8());
    }

    #[test]
    fn test_debug() {
        let s = "it's a \"test\" \\ \t\n\0\x7f caf\u{e9} \u{1f600}\u{200b}\u{301}\u{feff}";
        assert_eq!(
            format!("{:?}", BinString::new(s)),
            format!("BinString({s:?})")
        );

        let s = BinString::from_bytes(b"a\xed\xa0\x80\"\xff\n\xf0");
        assert_eq!(
            format!("{s:?}"),
            r#"BinString("a\xed\xa0\x80\"\xff\n\xf0")"#
        );
    }

    #[test]
    fn test_display() {
        let s = BinString::from_bytes(b"a\xffb");
        assert_eq!(format!("{s}").as_bytes(), b"a\xffb");
    }

    #[test]
    fn test_deref() {
        let mut s = BinString::new("hello");
        s.push_str(" world");
        assert_eq!(*s, "hello world");
    }

    #[test]
    fn test_extend_and_collect() {
        let mut s = BinString::new("hello");
        s.extend([' ', 'w']);
        s.extend(["or", "ld"]);
        assert_eq!(s.as_str(), "hello world");

        let s: BinString = ['h', 'i'].iter().collect();
        assert_eq!(s.as_str(), "hi");

        let s: BinString = ["hello", " ", "world"].into_iter().collect();
        assert_eq!(s.as_str(), "hello world");
    }
}
