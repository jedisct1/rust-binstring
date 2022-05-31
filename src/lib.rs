#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BinString(pub String);

impl BinString {
    pub fn new(s: impl Into<String>) -> Self {
        BinString(s.into())
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        BinString(unsafe { String::from_utf8_unchecked(bytes.into()) })
    }

    pub fn unwrap(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

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
        BinString::from_bytes(bytes)
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
