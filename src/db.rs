use leveldb_sys::*;

use crate::iter::OwnedIterator;
use crate::options::Options;
use crate::options::ReadOptions;
use crate::string::String as LevelDbString;
use std::ffi::CString;

#[derive(Debug)]
pub struct Db {
    ptr: *mut leveldb_t,

    _options: Options,
}

impl Db {
    pub fn open<P: Into<Vec<u8>>>(path: P, _options: Options) -> Result<Self, LevelDbString> {
        let path = CString::new(path).unwrap();
        let mut err_ptr = std::ptr::null_mut();

        let ptr = unsafe {
            let ptr = leveldb_open(
                _options.ptr(),
                path.as_bytes().as_ptr() as *const i8,
                &mut err_ptr,
            );
            if !err_ptr.is_null() {
                return Err(LevelDbString::from_ptr(err_ptr));
            }
            ptr
        };

        assert!(!ptr.is_null());

        Ok(Db { ptr, _options })
    }

    pub fn iter_owned(&mut self, options: &ReadOptions) -> OwnedIterator {
        unsafe {
            let ptr = leveldb_create_iterator(self.ptr, options.ptr());
            OwnedIterator::from_parts(ptr, self)
        }
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        unsafe {
            leveldb_close(self.ptr);
        }
    }
}
