use leveldb_sys as sys;
use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str::Utf8Error;

/// A LevelDB CString.
///
/// Like regular CStrings, it is a byte-array with no specified encoding.
/// The length is also stored alongside the pointer, so this supports interior NULs.
pub struct String {
    ptr: *mut c_char,
    len: usize,
}

impl String {
    /// Make a [`String`] from a ptr.
    ///
    /// This will use strlen to determine the string length.
    ///
    /// # Safety
    /// The pointer must be a malloc-ed C string from the leveldb C api.
    ///
    /// # Panics
    /// Panics if the ptr is null.
    pub unsafe fn from_ptr(ptr: *mut c_char) -> Self {
        Self::try_from_ptr(ptr).expect("ptr is null")
    }

    /// Make a [`String`] from a ptr.
    ///
    /// Fallibly make a string from a ptr.
    ///
    /// # Safety
    /// The pointer must be a malloc-ed C string from the leveldb C api.
    pub unsafe fn try_from_ptr(ptr: *mut c_char) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            let len = unsafe { CStr::from_ptr(ptr).to_bytes().len() };
            Some(Self { ptr, len })
        }
    }

    /// Make a [`String`] from a ptr and a len, fallibly.
    ///
    /// # Safety
    /// * The pointer must be a malloc-ed C string from the leveldb C api.
    /// * The length must be the length of the string, excluding the nul byte.
    pub unsafe fn try_from_ptr_len(ptr: *mut c_char, len: usize) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, len })
        }
    }

    /// Get the contents as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.cast(), self.len) }
    }

    /// Try to convert this into a str.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    /// Lossily convert this into a str.
    pub fn to_string_lossy(&self) -> Cow<str> {
        std::string::String::from_utf8_lossy(self.as_bytes())
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            sys::leveldb_free(self.ptr.cast());
        }
    }
}

impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.as_bytes().escape_ascii())
    }
}
