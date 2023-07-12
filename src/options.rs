use leveldb_sys as sys;

/// Leveldb Open options
#[derive(Debug)]
pub struct Options(pub(crate) *mut sys::leveldb_options_t);

impl Options {
    /// Make a new options object
    pub fn new() -> Self {
        let ptr = unsafe { sys::leveldb_options_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        unsafe { sys::leveldb_options_destroy(self.0) }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}

/// Options for reading keys
#[derive(Debug)]
pub struct ReadOptions(pub(crate) *mut sys::leveldb_readoptions_t);

impl ReadOptions {
    /// Make a new ReadOptions object
    pub fn new() -> Self {
        let ptr = unsafe { sys::leveldb_readoptions_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

impl Drop for ReadOptions {
    fn drop(&mut self) {
        unsafe {
            sys::leveldb_readoptions_destroy(self.0);
        }
    }
}

impl Default for ReadOptions {
    fn default() -> Self {
        ReadOptions::new()
    }
}
