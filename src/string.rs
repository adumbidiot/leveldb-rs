use std::borrow::Cow;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str::Utf8Error;

pub struct String(*mut c_char);

impl String {
    /// # Safety
    /// The pointer must be a malloc-ed c string from the leveldb c api.
    pub unsafe fn from_ptr(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }

    pub fn to_c_str(&self) -> &CStr {
        unsafe { &CStr::from_ptr(self.0) }
    }

    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        self.to_c_str().to_str()
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        self.to_c_str().to_string_lossy()
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
        write!(f, "{:#?}", self.to_string_lossy())
    }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}
