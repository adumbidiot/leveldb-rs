use std::borrow::Cow;
use std::os::raw::c_char;

pub struct OwnedSlice(Vec<c_char>);

impl OwnedSlice {
    pub fn from_slice(a: &[c_char]) -> Self {
        Self(a.to_vec())
    }

    pub fn as_slice(&self) -> &[c_char] {
        &self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*(self.0.as_ref() as *const [i8] as *const [u8]) }
    }

    pub fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
    }
}

impl std::fmt::Display for OwnedSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl std::fmt::Debug for OwnedSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}
