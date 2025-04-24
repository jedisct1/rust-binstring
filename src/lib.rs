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
//! Most operations in this library work at the byte level and are safe to use with
//! any binary data. However, some methods like `trim()` assume valid UTF-8 and should
//! only be used when you are certain the data is valid UTF-8.
//!
//! # Examples
//!
//! ```
//! use binstring::BinString;
//!
//! // Create from a string
//! let bin_str = BinString::new("hello");
//! assert_eq!(bin_str.as_str(), "hello");
//!
//! // Create from bytes
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
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
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
        use ops::Bound;
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.len(),
        };
        assert!(start <= end, "start must be less than or equal to end");
        assert!(
            end <= self.len(),
            "end must be less than or equal to length"
        );
        BinString::from_bytes(&self.as_bytes()[start..end])
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
        self.as_bytes()
            .windows(needle.len())
            .any(|window| window == needle)
    }

    /// Returns the index of the first occurrence of the given bytes.
    ///
    /// Returns `None` if the bytes are not found.
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
        self.as_bytes()
            .windows(needle.len())
            .position(|window| window == needle)
    }

    /// Returns the index of the last occurrence of the given bytes.
    ///
    /// Returns `None` if the bytes are not found.
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
    /// # Safety
    ///
    /// This method assumes that the string contains valid UTF-8. If the string
    /// contains invalid UTF-8, the behavior is undefined.
    ///
    /// # Examples
    ///
    /// ```
    /// use binstring::BinString;
    ///
    /// let s = BinString::new("  hello  ");
    /// assert_eq!(s.trim().as_str(), "hello");
    /// ```
    #[inline]
    pub fn trim(&self) -> BinString {
        BinString::new(self.0.trim())
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

impl Borrow<[u8]> for BinString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bin_str = BinString::new("hello");
        assert_eq!(bin_str.as_str(), "hello");
    }

    #[test]
    fn test_from_bytes() {
        let bytes = vec![104, 101, 108, 108, 111];
        let bin_str = BinString::from_bytes(bytes);
        assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_unwrap() {
        let bin_str = BinString::new("hello");
        let s = bin_str.unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_as_str() {
        let bin_str = BinString::new("hello");
        assert_eq!(bin_str.as_str(), "hello");
    }

    #[test]
    fn test_as_bytes() {
        let bin_str = BinString::new("hello");
        assert_eq!(bin_str.as_bytes(), &[104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_len() {
        let bin_str = BinString::new("hello");
        assert_eq!(bin_str.len(), 5);
    }

    #[test]
    fn test_is_empty() {
        let bin_str = BinString::new("");
        assert!(bin_str.is_empty());
    }

    #[test]
    fn test_concat() {
        let s1 = BinString::new("hello");
        let s2 = BinString::new(" world");
        assert_eq!(s1.concat(&s2).as_str(), "hello world");
    }

    #[test]
    fn test_slice() {
        let s = BinString::new("hello");
        assert_eq!(s.slice(1..4).as_str(), "ell");
    }

    #[test]
    #[should_panic]
    fn test_slice_out_of_bounds() {
        let s = BinString::new("hello");
        let _ = s.slice(10..20);
    }

    #[test]
    fn test_starts_with() {
        let s = BinString::new("hello");
        assert!(s.starts_with(&[104, 101])); // "he"
    }

    #[test]
    fn test_ends_with() {
        let s = BinString::new("hello");
        assert!(s.ends_with(&[108, 111])); // "lo"
    }

    #[test]
    fn test_contains() {
        let s = BinString::new("hello");
        assert!(s.contains(&[101, 108])); // "el"
    }

    #[test]
    fn test_find() {
        let s = BinString::new("hello");
        assert_eq!(s.find(&[101, 108]), Some(1)); // "el"
        assert_eq!(s.find(&[120]), None); // "x"
    }

    #[test]
    fn test_rfind() {
        let s = BinString::new("hello");
        assert_eq!(s.rfind(&[108]), Some(3)); // "l"
        assert_eq!(s.rfind(&[120]), None); // "x"
    }

    #[test]
    fn test_replace() {
        let s = BinString::new("hello");
        assert_eq!(s.replace(108, 120).as_bytes(), &[104, 101, 120, 120, 111]); // "hexxo"
    }

    #[test]
    fn test_trim() {
        let s = BinString::new("  hello  ");
        assert_eq!(s.trim().as_str(), "hello");
    }

    #[test]
    fn test_display() {
        let s = BinString::new("hello");
        assert_eq!(format!("{}", s), "hello");
    }

    #[test]
    fn test_from_str() {
        let s: BinString = "hello".into();
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_vec_u8() {
        let bytes = vec![104, 101, 108, 108, 111];
        let s: BinString = bytes.into();
        assert_eq!(s.as_bytes(), &[104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_from_slice_u8() {
        let bytes: &[u8] = &[104, 101, 108, 108, 111];
        let s: BinString = bytes.into();
        assert_eq!(s.as_bytes(), &[104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_default() {
        let s = BinString::default();
        assert!(s.is_empty());
    }

    #[test]
    fn test_deref() {
        let s = BinString::new("hello");
        assert_eq!(s.len(), 5);
        assert_eq!(s.capacity(), 5);
    }

    #[test]
    fn test_deref_mut() {
        let mut s = BinString::new("hello");
        s.push_str(" world");
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_borrow_str() {
        use std::borrow::Borrow;
        let s = BinString::new("hello");
        let borrowed: &str = s.borrow();
        assert_eq!(borrowed, "hello");
    }

    #[test]
    fn test_borrow_bytes() {
        use std::borrow::Borrow;
        let s = BinString::new("hello");
        let borrowed: &[u8] = s.borrow();
        assert_eq!(borrowed, &[104, 101, 108, 108, 111]);
    }

    #[test]
    fn test_extend_chars() {
        let mut s = BinString::new("hello");
        s.extend([' ', 'w', 'o', 'r', 'l', 'd']);
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_extend_str() {
        let mut s = BinString::new("hello");
        s.extend([" ", "world"]);
        assert_eq!(s.as_str(), "hello world");
    }

    #[test]
    fn test_from_iter_chars() {
        let s: BinString = ['h', 'e', 'l', 'l', 'o'].iter().collect();
        assert_eq!(s.as_str(), "hello");
    }

    #[test]
    fn test_from_iter_str() {
        let s: BinString = ["hello", " ", "world"].into_iter().collect();
        assert_eq!(s.as_str(), "hello world");
    }
}
