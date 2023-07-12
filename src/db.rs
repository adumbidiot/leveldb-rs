use leveldb_sys::*;

use crate::iter::OwnedIterator;
use crate::options::Options;
use crate::options::ReadOptions;
use crate::string::String as LevelDbString;
use std::ffi::CString;

/// A Leveldb Database
#[derive(Debug)]
pub struct Db {
    ptr: *mut leveldb_t,

    // TODO: Figure out why this is here
    #[allow(dead_code)]
    options: Options,
}

impl Db {
    /// Open a leveldb database.
    /// # Panics
    /// Panics if there is an interior NUL in the path.
    pub fn open(path: impl Into<Vec<u8>>, options: Options) -> Result<Self, LevelDbString> {
        let path = CString::new(path).expect("No interior NULs in path parameter");
        let mut err_ptr = std::ptr::null_mut();

        let ptr = unsafe {
            let ptr = leveldb_open(options.as_raw(), path.as_ptr(), &mut err_ptr);
            let err = LevelDbString::try_from_ptr(err_ptr);
            if let Some(err) = err {
                return Err(err);
            }
            ptr
        };

        assert!(!ptr.is_null());

        Ok(Db { ptr, options })
    }

    /// Get a value with a key.
    pub fn get(
        &mut self,
        options: &ReadOptions,
        key: &[u8],
    ) -> Result<Option<LevelDbString>, LevelDbString> {
        let mut value_len = 0;
        let mut err_ptr = std::ptr::null_mut();
        let ptr = unsafe {
            leveldb_get(
                self.ptr,
                options.0,
                key.as_ptr().cast(),
                key.len(),
                &mut value_len,
                &mut err_ptr,
            )
        };

        let value = unsafe { LevelDbString::try_from_ptr_len(ptr, value_len) };
        let err = unsafe { LevelDbString::try_from_ptr(err_ptr) };

        if let Some(err) = err {
            return Err(err);
        }

        Ok(value)
    }

    /// Iter all db keys
    pub fn iter_owned(&mut self, options: &ReadOptions) -> OwnedIterator {
        unsafe {
            let ptr = leveldb_create_iterator(self.ptr, options.as_raw());
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
