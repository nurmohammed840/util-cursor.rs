#![doc = include_str!("../README.md")]
#![no_std]

use core::fmt;

#[derive(Default)]
pub struct Cursor<T> {
    pub data: T,
    pub offset: usize,
}

impl<'a, T> Cursor<&'a [T]> {
    /// Returns remaining slice from the current offset.
    /// It doesn't change the offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use util_cursor::Cursor;
    ///
    /// let mut cursor = Cursor::new([1, 2].as_ref());
    ///
    /// assert_eq!(cursor.remaining_slice(), &[1, 2]);
    /// cursor.offset = 42;
    /// assert!(cursor.remaining_slice().is_empty());
    /// ```
    #[inline]
    pub fn remaining_slice(&self) -> &'a [T] {
        unsafe { self.data.get_unchecked(self.offset.min(self.data.len())..) }
    }

    /// Read slice from the current offset.
    ///
    /// # Example
    /// ```
    /// use util_cursor::Cursor;
    /// let mut view = Cursor::new([1, 2, 3].as_ref());
    ///
    /// assert_eq!(view.read_slice(2), Some([1, 2].as_ref()));
    /// assert_eq!(view.read_slice(3), None);
    /// ```
    #[inline]
    pub fn read_slice(&mut self, len: usize) -> Option<&'a [T]> {
        let total_len = self.offset + len;
        let slice = self.data.get(self.offset..total_len)?;
        self.offset = total_len;
        Some(slice)
    }
}

impl<T> Cursor<T> {
    /// Creates a new [`Cursor<T>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use util_cursor::Cursor;
    ///
    /// let cursor = Cursor::new([1, 2].as_ref());
    /// ```
    #[inline]
    pub const fn new(data: T) -> Self {
        Self { data, offset: 0 }
    }
}

impl<T> From<T> for Cursor<T> {
    #[inline]
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T: Clone> Clone for Cursor<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            offset: self.offset.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Cursor<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor")
            .field("data", &self.data)
            .field("offset", &self.offset)
            .finish()
    }
}
