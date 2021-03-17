use crate::{
    misc::{ BufferTooSmall, RangeBoundsExt },
    traits::{ ArrayRef, ArrayMut, ArrayAlloc, CanAlloc },
    std::{
        cmp::Ordering, ops::RangeBounds,
        fmt::{ self, Debug, Formatter },
        hash::{ Hash, Hasher },
        slice::{ Iter as SliceIter, IterMut as SliceIterMut }
    }
};


/// A wrapper for array types that exposes checked APIs only
pub struct Array<Wrapped> {
    /// The wrapped element
    wrapped: Wrapped
}
impl<Wrapped> Array<Wrapped> {
    /// Wraps an `array`
    pub const fn new(array: Wrapped) -> Self {
        Self { wrapped: array }
    }

    /// Returns the wrapped array
    #[inline(always)]
    pub fn into_inner(self) -> Wrapped {
        self.wrapped
    }
}
impl<Wrapped> AsRef<Array<Wrapped>> for Array<Wrapped> {
    fn as_ref(&self) -> &Array<Wrapped> {
        self
    }
}
impl<T, Wrapped> ArrayRef<T> for Array<Wrapped> where Wrapped: AsRef<[T]> {
    fn as_slice(&self) -> &[T] {
        self.wrapped.as_ref()
    }
    fn len(&self) -> usize {
        self.as_slice().len()
    }
    fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }
    fn get_n<Range>(&self, range: Range) -> Option<Array<&[T]>> where Range: RangeBounds<usize> {
        let slice = self.as_slice();
        let range = range.into_absolute(0, slice.len())?;
        slice.get(range).map(Array::new)
    }

    fn first(&self) -> Option<&T> {
        self.as_slice().first()
    }
    fn last(&self) -> Option<&T> {
        self.as_slice().last()
    }

    fn iter(&self) -> SliceIter<T> {
        self.as_slice().iter()
    }

    fn clone_to<Source>(&self, dest: &mut Source) -> Result<(), BufferTooSmall> where Source: ArrayMut<T>, T: Clone {
        // Validate length
        if self.len() > dest.len() {
            Err(BufferTooSmall)?;
        }

        // Clone the source elements to dest
        dest.iter_mut().zip(self.iter()).for_each(|(t, e)| *t = e.clone());
        Ok(())
    }
}
impl<T, Wrapped> ArrayMut<T> for Array<Wrapped> where Wrapped: AsRef<[T]> + AsMut<[T]> {
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.wrapped.as_mut()
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.as_slice_mut().get_mut(index)
    }
    fn get_n_mut<Range>(&mut self, range: Range) -> Option<Array<&mut [T]>> where Range: RangeBounds<usize> {
        let slice = self.as_slice_mut();
        let range = range.into_absolute(0, slice.len())?;
        slice.get_mut(range).map(Array::new)
    }

    fn first_mut(&mut self) -> Option<&mut T> {
        self.as_slice_mut().first_mut()
    }
    fn last_mut(&mut self) -> Option<&mut T> {
        self.as_slice_mut().last_mut()
    }

    fn iter_mut(&mut self) -> SliceIterMut<T> {
        self.as_slice_mut().iter_mut()
    }

    fn rotate_left(&mut self, count: usize) {
        // Avoid division by zero
        if self.is_empty() {
            return;
        }

        let count = count % self.len();
        self.as_slice_mut().rotate_left(count);
    }
    fn rotate_right(&mut self, count: usize) {
        // Avoid division by zero
        if self.is_empty() {
            return;
        }

        let count = count % self.len();
        self.as_slice_mut().rotate_right(count);
    }
    fn reverse(&mut self) {
        self.as_slice_mut().reverse()
    }
}
impl<T, Wrapped> ArrayAlloc<T> for Array<Wrapped> where Wrapped: AsRef<[T]> + AsMut<[T]> + CanAlloc<T> {
    type Error = Wrapped::Error;
    
    fn alloc_new() -> Result<Self, Self::Error> {
        Ok(Self::new(Wrapped::alloc_new()?))
    }
    fn alloc_clone<Source>(elements: &Source) -> Result<Self, Self::Error> where Source: ArrayRef<T>, T: Clone {
        let mut this = Self::alloc_new()?;
        this.push_n_back(elements)?;
        Ok(this)
    }
    
    fn grow_with(&mut self, len: usize, mut init: impl FnMut() -> T) -> Result<(), Self::Error> {
        for _ in 0 .. len.saturating_sub(self.len()) {
            self.push_back(init())?;
        }
        Ok(())
    }
    fn grow(&mut self, len: usize) -> Result<(), Self::Error> where T: Default {
        self.grow_with(len, T::default)
    }
    fn shrink(&mut self, len: usize) -> Result<(), Self::Error> {
        for _ in 0 .. self.len().saturating_sub(len) {
            let _ = self.wrapped.pop()?;
        }
        Ok(())
    }

    fn push_front(&mut self, element: T) -> Result<(), Self::Error> {
        self.push_back(element)?;
        self.rotate_right(1);
        Ok(())
    }
    fn push_n_front<Source>(&mut self, elements: &Source) -> Result<(), Self::Error>
        where Source: ArrayRef<T>, T: Clone
    {
        self.push_n_back(elements)?;
        self.rotate_right(elements.len());
        Ok(())
    }
    fn push_back(&mut self, element: T) -> Result<(), Self::Error> {
        self.wrapped.push(element)
    }
    fn push_n_back<Source>(&mut self, elements: &Source) -> Result<(), Self::Error>
        where Source: ArrayRef<T>, T: Clone
    {
        elements.iter().cloned().try_for_each(|e| self.push_back(e))
    }

    fn pop_front(&mut self) -> Result<Option<T>, Self::Error> {
        self.rotate_left(1);
        self.pop_back()
    }
    fn pop_n_front(&mut self, len: usize) -> Result<Option<Self>, Self::Error> {
        self.rotate_left(len);
        self.pop_n_back(len)
    }
    fn pop_back(&mut self) -> Result<Option<T>, Self::Error> {
        self.wrapped.pop()
    }
    fn pop_n_back(&mut self, len: usize) -> Result<Option<Self>, Self::Error> {
        // Don't mutate `self` if the operation cannot succeed
        if self.len() < len {
            return Ok(None)
        }

        // Move element by element into the new array
        let mut popped = Self::alloc_new()?;
        for _ in 0 .. len {
            let element = self.pop_back()?.expect("Failed to pop existing element?!");
            popped.push_back(element)?;
        }

        // Reverse the order in the new array and return it
        popped.reverse();
        Ok(Some(popped))
    }
}
// - MARK: Propagate common trait implementations
impl<Wrapped> Debug for Array<Wrapped> where Wrapped: Debug {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.wrapped.fmt(f)
    }
}
impl<Wrapped> Default for Array<Wrapped> where Wrapped: Default {
    fn default() -> Self {
        Self { wrapped: Wrapped::default() }
    }
}
impl<Wrapped> Copy for Array<Wrapped> where Wrapped: Copy {
    /* Copy is an intrinsic marker trait; no implementation required */
}
impl<Wrapped> Clone for Array<Wrapped> where Wrapped: Clone {
    fn clone(&self) -> Self {
        Self { wrapped: self.wrapped.clone() }
    }
}
impl<Wrapped> PartialEq for Array<Wrapped> where Wrapped: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.wrapped.eq(&other.wrapped)
    }
}
impl<Wrapped> Eq for Array<Wrapped> where Wrapped: Eq {
    /* Copy is a marker trait; no implementation required */
}
impl<Wrapped> PartialOrd for Array<Wrapped> where Wrapped: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.wrapped.partial_cmp(&other.wrapped)
    }
}
impl<Wrapped> Ord for Array<Wrapped> where Wrapped: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.wrapped.cmp(&other.wrapped)
    }
}
impl<Wrapped> Hash for Array<Wrapped> where Wrapped: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wrapped.hash(state)
    }
}
impl<Wrapped> IntoIterator for Array<Wrapped> where Wrapped: IntoIterator {
    type Item = Wrapped::Item;
    type IntoIter = Wrapped::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.wrapped.into_iter()
    }
}