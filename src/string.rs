use std::borrow::Cow;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str::Utf8Error;

/// A LevelDB CString. Like regular CStrings, it is a byte-array.
pub struct String(*mut c_char);

impl String {
    /// Make a String from a ptr.
    ///
    /// # Safety
    /// The pointer must be a malloc-ed c string from the leveldb c api.
    ///
    /// # Panics
    /// Panics if the ptr is null.
    pub unsafe fn from_ptr(ptr: *mut c_char) -> Self {
        Self::try_from_ptr(ptr).expect("Non Null LevelDB CString Pointer")
    }

    /// Fallibly make a string from a ptr.
    ///
    /// # Safety
    /// The pointer must be a malloc-ed c string from the leveldb c api.
    pub unsafe fn try_from_ptr(ptr: *mut c_char) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr))
        }
    }

    /// Get the contents as a `&CStr`.
    pub fn as_c_str(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.0) }
    }

    /// Try to convert this into a str.
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        self.as_c_str().to_str()
    }

    /// Lossily convert this into a str.
    pub fn to_string_lossy(&self) -> Cow<str> {
        self.as_c_str().to_string_lossy()
    }
}

impl Drop for String {
    fn drop(&mut self) {
        unsafe {
            leveldb_sys::leveldb_free(self.0 as *mut c_void);
        }
    }
}

impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}
