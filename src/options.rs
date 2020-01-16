use leveldb_sys::*;

#[derive(Debug)]
pub struct Options(*mut leveldb_options_t);

impl Options {
    pub fn new() -> Self {
        let ptr = unsafe { leveldb_options_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }

    pub fn ptr(&self) -> *const leveldb_options_t {
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

#[derive(Debug)]
pub struct ReadOptions(*mut leveldb_readoptions_t);

impl ReadOptions {
    pub fn new() -> Self {
        let ptr = unsafe { leveldb_readoptions_create() };
        assert!(!ptr.is_null());
        Self(ptr)
    }

    pub fn ptr(&self) -> *const leveldb_readoptions_t {
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
