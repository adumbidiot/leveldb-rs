use crate::db::Db;
use crate::slice::OwnedSlice;
use leveldb_sys::*;

pub struct OwnedIterator<'a>(*mut leveldb_iterator_t, &'a Db);

impl<'a> OwnedIterator<'a> {
    ///# Safety
    /// ptr must be an pointer to a leveldb iterator from the c api with a lifetime bounded by a wrapped database from this library
    pub unsafe fn from_parts(ptr: *mut leveldb_iterator_t, db: &'a Db) -> Self {
        assert!(!ptr.is_null());

        leveldb_iter_seek_to_first(ptr);

        Self(ptr, db)
    }

    pub fn valid(&self) -> bool {
        unsafe { leveldb_iter_valid(self.0) > 0 }
    }

    pub fn key(&self) -> Option<OwnedSlice> {
        let mut len = 0;
        unsafe {
            let ptr = leveldb_iter_key(self.0, &mut len);
            if !ptr.is_null() {
                Some(OwnedSlice::from_slice(std::slice::from_raw_parts(
                    ptr,
                    len as usize,
                )))
            } else {
                None
            }
        }
    }

    pub fn value(&self) -> Option<OwnedSlice> {
        let mut len = 0;
        unsafe {
            let ptr = leveldb_iter_value(self.0, &mut len);
            if !ptr.is_null() {
                Some(OwnedSlice::from_slice(std::slice::from_raw_parts(
                    ptr,
                    len as usize,
                )))
            } else {
                None
            }
        }
    }
}

impl<'a> Drop for OwnedIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            leveldb_iter_destroy(self.0);
        }
    }
}

impl<'a> Iterator for OwnedIterator<'a> {
    type Item = (OwnedSlice, OwnedSlice);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid() {
            return None;
        }

        let key = self.key()?;
        let value = self.value()?;

        unsafe {
            leveldb_iter_next(self.0);
        }

        Some((key, value))
    }
}
