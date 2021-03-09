use crate::{
    misc::BufferTooSmall, wrapper::Array,
    std::{
        fmt::Debug, ops::RangeBounds,
        slice::{ Iter as SliceIter, IterMut as SliceIterMut }
    }
};
#[cfg(feature = "std")]
use crate::misc::WillPanic;


/// A trait for referencable linear array types
pub trait ArrayRef<T> {
    /// The underlying elements as slice
    fn as_slice(&self) -> &[T];
    /// The length of the wrapped elements
    fn len(&self) -> usize;
    /// Whether `self` is empty or not
    fn is_empty(&self) -> bool;

    /// Gets an element
    fn get(&self, index: usize) -> Option<&T>;
    /// Gets a subrange
    fn get_n<Range>(&self, range: Range) -> Option<Array<&[T]>> where Range: RangeBounds<usize>;

    /// Returns a reference to the first element
    fn first(&self) -> Option<&T>;
    /// Returns a reference to the last element
    fn last(&self) -> Option<&T>;

    /// Returns an iterator that references the elements
    fn iter(&self) -> SliceIter<T>;

    /// Clones `self` to `target`
    fn clone_to<Target>(&self, target: &mut Target) -> Result<(), BufferTooSmall> where Target: ArrayMut<T>, T: Clone;
}


/// A trait for mutalby referencable linear array types
pub trait ArrayMut<T>: ArrayRef<T> {
    /// The underlying element as mutable slice
    fn as_slice_mut(&mut self) -> &mut [T];

    /// Gets a mutable reference to an element
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    /// Gets a mutable subrange
    fn get_n_mut<Range>(&mut self, range: Range) -> Option<Array<&mut [T]>> where Range: RangeBounds<usize>;

    /// Returns a mutable reference to the first element
    fn first_mut(&mut self) -> Option<&mut T>;
    /// Returns a mutable reference to the last element
    fn last_mut(&mut self) -> Option<&mut T>;

    /// Returns an iterator that mutably references the elements
    fn iter_mut(&mut self) -> SliceIterMut<T>;

    /// Rotates the elements left by `count` fields
    fn rotate_left(&mut self, count: usize);
    /// Rotates the elements right by `count` fields
    fn rotate_right(&mut self, count: usize);
    /// Reverses the order of elements in the slice
    fn reverse(&mut self);
}


/// A trait for allocatable/resizeable linear array types
pub trait ArrayAlloc<T>: ArrayMut<T> + Sized {
    /// An alloc related error
    type Error: Debug;
    
    /// Creates a newly allocated instance of `Self`
    fn alloc_new() -> Result<Self, Self::Error>;
    /// Clones `source` into a newly allocated instance of `Self`
    fn alloc_clone<Source>(source: &Source) -> Result<Self, Self::Error> where Source: ArrayRef<T>, T: Clone;

    /// Grows `self` to the given capacity if the current length is smaller than `len` and inits new elements using `init`
    fn grow_with(&mut self, len: usize, init: impl FnMut() -> T) -> Result<(), Self::Error>;
    /// Grows `self` to the given capacity if the current length is smaller than `len` and inits new elements using
    /// `Default`
    fn grow(&mut self, len: usize) -> Result<(), Self::Error> where T: Default;
    /// Shrinks `self` to the given capacity if the current length is larger than `len`
    fn shrink(&mut self, len: usize) -> Result<(), Self::Error>;

    /// Pushes an `element` to the front of `self`
    fn push_front(&mut self, element: T) -> Result<(), Self::Error>;
    /// Pushes some `elements` to the front of `self`
    fn push_n_front<Source>(&mut self, elements: &Source) -> Result<(), Self::Error>
        where Source: ArrayRef<T>, T: Clone;
    /// Pushes an `element` to the front of `self`
    fn push_back(&mut self, element: T) -> Result<(), Self::Error>;
    /// Pushes some `elements` to the front of `self`
    fn push_n_back<Source>(&mut self, elements: &Source) -> Result<(), Self::Error> where Source: ArrayRef<T>, T: Clone;

    /// Pops an `element` from the front of `self`
    fn pop_front(&mut self) -> Result<Option<T>, Self::Error>;
    /// Pops multiple `elements` from the front of `self`
    fn pop_n_front(&mut self, len: usize) -> Result<Option<Self>, Self::Error>;
    /// Pops an `element` from the back of `self`
    fn pop_back(&mut self) -> Result<Option<T>, Self::Error>;
    /// Pops multiple `elements` from the back of `self`
    fn pop_n_back(&mut self, len: usize) -> Result<Option<Self>, Self::Error>;
}


/// An infallible/panicking variant of `ArrayAlloc`
///
///  - Note: This trait adopts Rust's "panic on allocation failure" policy. While this trait reintroduces a panic cause,
///    it's usually much more convenient to use – especially for `std`-types which use `WillPanic` anyway.
pub trait ArrayAllocPanic<T>: ArrayAlloc<T> {
    /// Creates a newly allocated instance of `Self`
    fn alloc_new() -> Self;
    /// Clones `source` into a newly allocated instance of `Self`
    fn alloc_clone<Source>(source: &Source) -> Self where Source: ArrayRef<T>, T: Clone;

    /// Grows `self` to the given capacity if the current length is smaller than `len` and inits new elements using `init`
    fn grow_with(&mut self, len: usize, init: impl FnMut() -> T);
    /// Grows `self` to the given capacity if the current length is smaller than `len` and inits new elements using
    /// `Default`
    fn grow(&mut self, len: usize) where T: Default;
    /// Shrinks `self` to the given capacity if the current length is larger than `len`
    fn shrink(&mut self, len: usize);

    /// Pushes an `element` to the front of `self`
    fn push_front(&mut self, element: T);
    /// Pushes some `elements` to the front of `self`
    fn push_n_front<Source>(&mut self, elements: &Source) where Source: ArrayRef<T>, T: Clone;
    /// Pushes an `element` to the front of `self`
    fn push_back(&mut self, element: T);
    /// Pushes some `elements` to the front of `self`
    fn push_n_back<Source>(&mut self, elements: &Source) where Source: ArrayRef<T>, T: Clone;

    /// Pops an `element` from the front of `self`
    fn pop_front(&mut self) -> Option<T>;
    /// Pops multiple `elements` from the front of `self`
    fn pop_n_front(&mut self, len: usize) -> Option<Self>;
    /// Pops an `element` from the back of `self`
    fn pop_back(&mut self) -> Option<T>;
    /// Pops multiple `elements` from the back of `self`
    fn pop_n_back(&mut self, len: usize) -> Option<Self>;
}
impl<T, Array> ArrayAllocPanic<T> for Array where Array: ArrayAlloc<T> {
    fn alloc_new() -> Self {
        <Self as ArrayAlloc<T>>::alloc_new().expect("Allocation error")
    }
    fn alloc_clone<Source>(elements: &Source) -> Self where Source: ArrayRef<T>, T: Clone {
        <Self as ArrayAlloc<T>>::alloc_clone(elements).expect("Allocation error")
    }

    fn grow_with(&mut self, len: usize, init: impl FnMut() -> T) {
        <Self as ArrayAlloc<T>>::grow_with(self, len, init).expect("Allocation error")
    }
    fn grow(&mut self, len: usize) where T: Default {
        <Self as ArrayAlloc<T>>::grow(self, len).expect("Allocation error")
    }
    fn shrink(&mut self, len: usize) {
        <Self as ArrayAlloc<T>>::shrink(self, len).expect("Allocation error")
    }

    fn push_front(&mut self, element: T) {
        <Self as ArrayAlloc<T>>::push_front(self, element).expect("Allocation error")
    }
    fn push_n_front<Source>(&mut self, elements: &Source) where Source: ArrayRef<T>, T: Clone {
        <Self as ArrayAlloc<T>>::push_n_front(self, elements).expect("Allocation error")
    }
    fn push_back(&mut self, element: T) {
        <Self as ArrayAlloc<T>>::push_back(self, element).expect("Allocation error")
    }
    fn push_n_back<Source>(&mut self, elements: &Source) where Source: ArrayRef<T>, T: Clone {
        <Self as ArrayAlloc<T>>::push_n_back(self, elements).expect("Allocation error")
    }

    fn pop_front(&mut self) -> Option<T> {
        <Self as ArrayAlloc<T>>::pop_front(self).expect("Allocation error")
    }
    fn pop_n_front(&mut self, len: usize) -> Option<Self> {
        <Self as ArrayAlloc<T>>::pop_n_front(self, len).expect("Allocation error")
    }
    fn pop_back(&mut self) -> Option<T> {
        <Self as ArrayAlloc<T>>::pop_back(self).expect("Allocation error")
    }
    fn pop_n_back(&mut self, len: usize) -> Option<Self> {
        <Self as ArrayAlloc<T>>::pop_n_back(self, len).expect("Allocation error")
    }
}


/// A trait for types that can perform stack-like memory allocation
pub trait CanAlloc<T>: Sized {
    /// An allocation related error
    type Error: Debug;

    /// Creates a new potentially allocated instance of `Self`
    fn alloc_new() -> Result<Self, Self::Error>;

    /// Pushes an `element` to the end of `self`
    fn push(&mut self, element: T) -> Result<(), Self::Error>;
    /// Pops an `element` from the end of `self` if any
    fn pop(&mut self) -> Result<Option<T>, Self::Error>;
}
#[cfg(feature = "std")]
impl<T> CanAlloc<T> for Vec<T> {
    type Error = WillPanic;

    fn alloc_new() -> Result<Self, Self::Error> {
        Ok(Self::new())
    }

    fn push(&mut self, element: T) -> Result<(), Self::Error> {
        Ok(self.push(element))
    }
    fn pop(&mut self) -> Result<Option<T>, Self::Error> {
        Ok(self.pop())
    }
}