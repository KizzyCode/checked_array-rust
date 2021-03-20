use crate::{
    misc::BufferTooSmall, traits::CanAlloc,
    std::{ mem, cmp }
};


/// A wrapper that implements `CanAlloc` over a preallocated buffer
pub struct Preallocated<Buffer> {
    /// The wrapped buffer
    buffer: Buffer,
    /// The amount of bytes used
    used: usize
}
impl<Buffer> Preallocated<Buffer> {
    /// Create a new `Preallocated` instance by wrapping `buffer`
    pub const fn new(buffer: Buffer) -> Self {
        Self::new_with_used(buffer, 0)
    }
    /// Create a new `Preallocated` instance by wrapping `buffer` and sets the amount of used bytes to `used`
    ///
    /// __Discussion:__ If `used` is greated than `buffer.len()`, it will be silently capped to `buffer.len()` wherever
    /// necessary
    pub const fn new_with_used(buffer: Buffer, used: usize) -> Self {
        Self { buffer, used }
    }

    /// Returns the wrapped buffer
    #[inline(always)]
    pub fn into_inner(self) -> Buffer {
        self.buffer
    }
}
impl<Buffer, T> AsRef<[T]> for Preallocated<Buffer> where Buffer: AsRef<[T]> {
    fn as_ref(&self) -> &[T] {
        // Cap `used` to the buffer size because we cannot ensure that the buffer has not been resized somewhere else
        let buffer = self.buffer.as_ref();
        let used = cmp::min(self.used, buffer.len());
        
        // Take the used subslice
        &buffer[..used]
    }
}
impl<Buffer, T> AsMut<[T]> for Preallocated<Buffer> where Buffer: AsMut<[T]> {
    fn as_mut(&mut self) -> &mut [T] {
        // Cap `used` to the buffer size because we cannot ensure that the buffer has not been resized somewhere else
        let buffer = self.buffer.as_mut();
        let used = cmp::min(self.used, buffer.len());
        
        // Take the used subslice
        &mut buffer[..used]
    }
}
impl<Buffer, T> CanAlloc<T> for Preallocated<Buffer> where Buffer: AsRef<[T]> + AsMut<[T]>, T: Default {
    type Error = BufferTooSmall;

    /// __Warning:__ This function will always fail because we cannot create a preallocated out of nothing
    fn alloc_new() -> Result<Self, Self::Error> {
        Err(BufferTooSmall)
    }

    fn push(&mut self, element: T) -> Result<(), Self::Error> {
        // Cap `used` to the buffer size because we cannot ensure that the buffer has not been resized somewhere else
        let buffer = self.buffer.as_mut();
        self.used = cmp::min(self.used, buffer.len());

        // Ensure that the buffer is not full
        let next = match self.used.checked_add(1) {
            Some(next) if next <= buffer.len() => next,
            _ => Err(BufferTooSmall)?
        };

        // Append the element
        buffer[self.used] = element;
        self.used = next;
        Ok(())
    }

    fn pop(&mut self) -> Result<Option<T>, Self::Error> {
        // Cap `used` to the buffer size because we cannot ensure that the buffer has not been resized somewhere else
        let buffer = self.buffer.as_mut();
        self.used = cmp::min(self.used, buffer.len());

        // Validate that the used buffer is not empty
        let last = match self.used.checked_sub(1) {
            Some(last) => last,
            None => return Ok(None)
        };

        // Take the last element and replace it with a default element
        let element = mem::replace(&mut buffer[last], T::default());
        self.used = last;
        Ok(Some(element))
    }
}