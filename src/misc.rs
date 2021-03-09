use crate::std::{
    fmt::{ self, Display, Formatter },
    ops::{ Range, RangeBounds, Bound }
};
#[cfg(feature = "std")]
use crate::std::error::Error;


/// An error indicating that a buffer is too small
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferTooSmall;
impl Display for BufferTooSmall {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Buffer is too small")
    }
}
#[cfg(feature = "std")]
impl Error for BufferTooSmall {}


/// An error which indicates that an implementation will always panic instead of returning an error
#[derive(Debug)]
#[non_exhaustive]
pub enum WillPanic {}


/// An extension to the range bounds trait
pub trait RangeBoundsExt<T> {
    /// Computes an absolute range from `self` using `default_start` and `default_end` as hints if the range is
    /// (partially) open
    fn into_absolute(self, default_start: T, default_end: T) -> Option<Range<T>>;
}
impl<T> RangeBoundsExt<usize> for T where T: RangeBounds<usize> {
    fn into_absolute(self, default_start: usize, default_end: usize) -> Option<Range<usize>> {
        // Translate start and end
        let start = match self.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => start.checked_add(1)?,
            Bound::Unbounded => default_start
        };
        let end = match self.end_bound() {
            Bound::Excluded(end) => *end,
            Bound::Included(end) => end.checked_add(1)?,
            Bound::Unbounded => default_end
        };
        
        match start <= end {
            true => Some(start..end),
            false => None
        }
    }
}