#![cfg_attr(feature = "no_std", no_std)]

/// Reexports core or std as `crate::std`
pub(in crate) mod std {
    #[cfg(not(feature = "std"))]
    pub use core::*;

    #[cfg(feature = "std")]
    pub use std::*;
}


/// Miscellaneous stuff
pub mod misc;

/// Byte traits exposing a checked API only
mod traits;
pub use crate::traits::{ ArrayRef, ArrayMut, ArrayAlloc, ArrayAllocPanic };

/// A generic wrapper that implements the `Bytes*`-traits for the underlying element
mod wrapper;
pub use crate::wrapper::Array;