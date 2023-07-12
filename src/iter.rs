use crate::db::Db;
use leveldb_sys as sys;

pub struct OwnedIterator<'a>(*mut sys::leveldb_iterator_t, &'a Db);

impl<'a> OwnedIterator<'a> {
    ///# Safety
    /// ptr must be an pointer to a leveldb iterator from the c api with a lifetime bounded by a wrapped database from this library
    pub unsafe fn from_parts(ptr: *mut sys::leveldb_iterator_t, db: &'a Db) -> Self {
        assert!(!ptr.is_null());

        sys::leveldb_iter_seek_to_first(ptr);

        Self(ptr, db)
    }

    pub fn valid(&self) -> bool {
        unsafe { sys::leveldb_iter_valid(self.0) > 0 }
    }

    /// Fairly certain this should be &mut. TODO: Fix upstream.
    #[allow(clippy::unnecessary_mut_passed)]
    pub fn key(&self) -> Option<Vec<u8>> {
        let mut len = 0;
        unsafe {
            let ptr = sys::leveldb_iter_key(self.0, &mut len);
            if !ptr.is_null() {
                Some(std::slice::from_raw_parts(ptr.cast(), len).to_vec())
            } else {
                None
            }
        }
    }

    /// Fairly certain this should be &mut. TODO: Fix upstream.
    #[allow(clippy::unnecessary_mut_passed)]
    pub fn value(&self) -> Option<Vec<u8>> {
        let mut len = 0;
        unsafe {
            let ptr = sys::leveldb_iter_value(self.0, &mut len);
            if !ptr.is_null() {
                Some(std::slice::from_raw_parts(ptr.cast(), len).to_vec())
            } else {
                None
            }
        }
    }
}

impl<'a> Drop for OwnedIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            sys::leveldb_iter_destroy(self.0);
        }
    }
}

impl<'a> Iterator for OwnedIterator<'a> {
    type Item = (Vec<u8>, Vec<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.valid() {
            return None;
        }

        let key = self.key()?;
        let value = self.value()?;

        unsafe {
            sys::leveldb_iter_next(self.0);
        }

        Some((key, value))
    }
}
