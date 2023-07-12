use leveldb_sys::*;

/// Leveldb Open options
#[derive(Debug)]
pub struct Options(pub(crate) *mut leveldb_options_t);

impl Options {
    /// Make a new options object
    pub fn new() -> Self {
        let ptr = unsafe { leveldb_options_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }

    /// Get the inner pointer
    pub fn as_raw(&self) -> *const leveldb_options_t {
        self.0
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        unsafe { leveldb_options_destroy(self.0) }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for reading keys
#[derive(Debug)]
pub struct ReadOptions(pub(crate) *mut leveldb_readoptions_t);

impl ReadOptions {
    /// Make a new ReadOptions object
    pub fn new() -> Self {
        let ptr = unsafe { leveldb_readoptions_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }

    /// Get the inner pointer
    pub fn as_raw(&self) -> *const leveldb_readoptions_t {
        self.0
    }
}

impl Drop for ReadOptions {
    fn drop(&mut self) {
        unsafe {
            leveldb_readoptions_destroy(self.0);
        }
    }
}

impl Default for ReadOptions {
    fn default() -> Self {
        ReadOptions::new()
    }
}
