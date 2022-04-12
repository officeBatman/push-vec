//! Provides the `PushVec<T>` type, which is a vector that cannot be popped
//! from.
//!
//! This is useful for when you want to continue using a vector while keeping
//! references to its contents.
//!
//! # Example
//! ```
//! use push_vec::prelude::*;
//! let mut vec = push_vec![];
//! let x: &mut i32 = vec.push(1);
//! // We are holding a reference to an element, but we can still use the vector.
//! vec.push(2);
//! *x = 3;
//! assert_eq!(vec.into_vec(), vec![3, 2]);
//! ```

use std::slice::{self, SliceIndex};
use std::ops::{Index, IndexMut};
use std::iter::{FromIterator, IntoIterator};

/// A vector-like type that supports only push operations.
///
/// # Example
/// ```
/// use push_vec::prelude::*;
/// let mut vec = push_vec![];
/// let x: &mut i32 = vec.push(1);
/// // We are holding a reference to an element, but we can still use the vector.
/// vec.push(2);
/// *x = 3;
/// assert_eq!(vec.into_vec(), vec![3, 2]);
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PushVec<T>(Vec<T>);

impl<T> PushVec<T> {
    /// Creates a new, empty `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec: PushVec<i32> = PushVec::new();
    /// ```
    #[inline]
    pub const fn new() -> Self {
        PushVec(Vec::new())
    }

    /// Creates a `PushVec<T>` from a `Vec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let vec = vec![1, 2, 3];
    /// let mut vec = PushVec::from_vec(vec);
    /// ```
    #[inline]
    pub const fn from_vec(vec: Vec<T>) -> Self {
        PushVec(vec)
    }

    /// Cheaply converts a `PushVec<T>` into a `Vec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let vec1 = push_vec![1, 2, 3];
    /// let vec2 = vec![1, 2, 3];
    /// assert_eq!(vec1.into_vec(), vec2);
    /// ```
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }

    /// Returns an immutable reference to the underlying `Vec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let vec1 = push_vec![1, 2, 3];
    /// let vec2 = vec![1, 2, 3];
    /// assert_eq!(vec1.as_vec(), &vec2);
    /// ```
    #[inline]
    pub const fn as_vec(&self) -> &Vec<T> {
        &self.0
    }

    /// Returns the length of the `PushVec<T>`.
    /// This is the same as `Vec<T>::len()`.
    /// 
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let vec = push_vec![1, 2, 3];
    /// assert_eq!(vec.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the `PushVec<T>` is empty.
    /// This is the same as `Vec<T>::is_empty()`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![];
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a mutable reference to the elements, and doesn't borrow the
    /// `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// let slice = vec.as_mut_slice();
    /// // We can still push some elements
    /// vec.push(4);
    /// // And also use the slice
    /// slice[0] = 5;
    /// ```
    #[inline]
    pub fn as_mut_slice<'vec, 'a>(&'vec mut self) -> &'a mut [T]
        where Self: 'a,
    {
        unsafe { slice::from_raw_parts_mut(self.0.as_mut_ptr(), self.0.len()) }
    }

    /// Returns a reference to the elements, and doesn't borrow the `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// let slice = vec.as_slice();
    /// // We can still push some elements
    /// vec.push(4);
    /// // And also use the slice
    /// println!("{}", slice[0]);
    /// ```
    #[inline]
    pub fn as_slice<'vec, 'a>(&'vec self) -> &'a [T]
        where Self: 'a,
    {
        unsafe { slice::from_raw_parts(self.0.as_ptr(), self.0.len()) }
    }

    /// Returns a mutable reference to the element at the given index, and
    /// doesn't borrow the `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// let x: &mut i32 = vec.get_mut(1).unwrap();
    /// *x = 4;
    /// assert_eq!(vec.into_vec(), vec![1, 4, 3]);
    /// ```
    #[inline]
    pub fn get_mut<'vec, 'a>(&'vec mut self, index: usize) -> Option<&'a mut T>
        where Self: 'a,
    {
        self.as_mut_slice().get_mut(index)
    }

    /// Returns a reference to the element at the given index, and doesn't
    /// borrow the `PushVec<T>`.
    #[inline]
    pub fn get<'vec, 'a>(&'vec self, index: usize) -> Option<&'a T>
        where Self: 'a,
    {
        self.as_slice().get(index)
    }

    /// Returns an iterator over the elements of the `PushVec<T>`.
    /// Doesn't borrow the `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// for x in vec.iter() {
    ///    println!("{}", x);
    ///    vec.push(4);
    /// }
    /// ```
    #[inline]
    pub fn iter<'vec, 'a>(&'vec self) -> impl Iterator<Item = &'a T>
        where Self: 'a,
    {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the elements of the `PushVec<T>`.
    /// Doesn't borrow the `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// for x in vec.iter_mut() {
    ///   *x += 1;
    ///   vec.push(5);
    /// }
    /// ```
    #[inline]
    pub fn iter_mut<'vec, 'a>(&mut self) -> impl Iterator<Item = &'a mut T>
        where Self: 'a,
    {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.0.into_iter()
    }

    /// Pushes an element to the back of the `PushVec<T>`.
    /// Returns a mutable reference to the pushed element.
    /// Doesn't borrow the `PushVec<T>`.
    ///
    /// # Example
    /// ```
    /// use push_vec::prelude::*;
    /// let mut vec = push_vec![1, 2, 3];
    /// let x = vec.push(4);
    /// let y = vec.push(5);
    /// assert_eq!(vec, push_vec![1, 2, 3, 4, 5]);
    /// *x = 6;
    /// *y = 7;
    /// assert_eq!(vec, push_vec![1, 2, 3, 6, 7]);
    /// ```
    #[inline]
    pub fn push<'vec, 'a>(&'vec mut self, item: T) -> &'a mut T 
        where Self: 'a,
    {
        self.0.push(item);
        // This is safe because elements are never dropped before the vector is
        // and the reference surely exists.
        unsafe {
            let ptr = self.0.as_mut_ptr().offset(self.0.len() as isize - 1);
            &mut *ptr
        }
    }
}

impl<T> AsRef<[T]> for PushVec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> AsMut<[T]> for PushVec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for PushVec<T> {
    fn from(vec: Vec<T>) -> Self {
        PushVec::from_vec(vec)
    }
}

impl<T> From<PushVec<T>> for Vec<T> {
    fn from(push_vec: PushVec<T>) -> Self {
        push_vec.into_vec()
    }
}

impl<T> FromIterator<T> for PushVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        PushVec::from_vec(Vec::from_iter(iter))
    }
}

impl<T> Extend<T> for PushVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl<T, I> Index<I> for PushVec<T>
    where I: SliceIndex<[T]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &I::Output {
        &self.0[index]
    }
}

impl<T, I> IndexMut<I> for PushVec<T>
    where I: SliceIndex<[T]>
{
    fn index_mut(&mut self, index: I) -> &mut I::Output {
        &mut self.0[index]
    }
}


/// A macro for creating a `PushVec` from a list of elements.
#[macro_export]
macro_rules! push_vec {
    [$($x:expr),*] => {
        {
            PushVec::from_vec(vec![$($x),*])
        }
    };
    [$x:expr; $n:expr] => {
        {
            PushVec::from_vec(vec![$x; $n])
        }
    };
}

pub mod prelude {
    pub use super::{
        PushVec,
        push_vec
    };
}

