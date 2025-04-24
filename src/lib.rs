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
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl AsRef<str> for BinString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for BinString {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl From<String> for BinString {
    fn from(s: String) -> Self {
        BinString::new(s)
    }
}

impl From<BinString> for String {
    fn from(s: BinString) -> Self {
        s.0
    }
}

impl From<&str> for BinString {
    fn from(s: &str) -> Self {
        BinString::new(s.to_string())
    }
}

impl From<&[u8]> for BinString {
    fn from(bytes: &[u8]) -> Self {
        BinString::from_bytes(bytes.to_vec())
    }
}

impl From<Vec<u8>> for BinString {
    fn from(bytes: Vec<u8>) -> Self {
        BinString::from_bytes(bytes)
    }
}

impl From<&Vec<u8>> for BinString {
    fn from(bytes: &Vec<u8>) -> Self {
        BinString::from_bytes(bytes.to_owned())
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
}
