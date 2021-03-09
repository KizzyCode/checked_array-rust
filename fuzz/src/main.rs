#![no_main]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate libfuzzer_sys;

mod args_array_ref;
mod args_array_mut;
mod args_array_alloc;

use crate::{
    args_array_ref::{
        ArgsAsSlice, ArgsLen, ArgsIsEmpty, ArgsGet, ArgsGetN,
        ArgsFirst, ArgsLast, ArgsIter, ArgsCloneTo
    },
    args_array_mut::{ 
        ArgsAsSliceMut, ArgsGetMut, ArgsGetNMut, ArgsFirstMut, ArgsLastMut, ArgsIterMut,
        ArgsRotateLeft, ArgsRotateRight, ArgsReverse
    },
    args_array_alloc::{
        ArgsAllocNew, ArgsClone, ArgsGrowWith, ArgsGrow, ArgsShrink,
        ArgsPushFront, ArgsPushNFront, ArgsPushBack, ArgsPushNBack,
        ArgsPopFront, ArgsPopNFront, ArgsPopBack, ArgsPopNBack
    }
};
use checked_array::{ ArrayAllocPanic, Array };
use arbitrary::{ Arbitrary, Result, Unstructured };
use std::{
    any, env,
    fmt::{ self, Debug, Formatter }
};


lazy_static! {
    /// The maximum allocation limit
    static ref ALLOC_MAX: usize = {
        let limit = env::var("FUZZ_ALLOC_MAX").ok()
            .map(|s| usize::from_str_radix(&s, 10).expect("Invalid value for FUZZ_ALLOC_MAX"));
        limit.unwrap_or(16 * 1024 * 1024)
    };
}


/// An `Array*` fn call
pub trait ArrayFnCall {
    /// Calls `self` on `array`
    fn call(&mut self, array: &mut Array<Vec<u8>>);
}


/// A type erased `Array*` fn call
pub struct AnyArrayFnCall {
    /// The name of the call
    name: &'static str,
    /// The call itself
    call: Box<dyn ArrayFnCall>
}
impl AnyArrayFnCall {
    /// Creates a new type erased array fn call
    pub fn new<T>(call: T) -> Self where T: ArrayFnCall + 'static {
        let name = any::type_name::<T>();
        let call = Box::new(call);
        Self { name, call }
    }

    /// Applies the call to `array`
    pub fn call(&mut self, array: &mut Array<Vec<u8>>) {
        self.call.as_mut().call(array)
    }
}
impl Debug for AnyArrayFnCall {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyArrayFnCall")
            .field("name", &self.name)
            .field("call", &"Box<dyn ArrayFnCall>")
            .finish()
    }
}


/// All possible array fn calls
#[derive(Debug)]
pub struct ArrayFnCalls {
    /// All calls
    pub calls: Vec<AnyArrayFnCall>
}
impl<'a> Arbitrary<'a> for ArrayFnCalls {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        /// Creates an arbitrary call of type `T` from `u`
        fn arbitrary_call<'a, T>(u: &mut Unstructured<'a>) -> Result<AnyArrayFnCall>
            where T: Arbitrary<'a> + ArrayFnCall + 'static
        {
            let call = T::arbitrary(u)?;
            Ok(AnyArrayFnCall::new(call))
        }

        // The array fn constructors
        let mut constructors: Vec<Box<dyn FnMut(&mut Unstructured<'a>) -> Result<AnyArrayFnCall>>> = vec![
            Box::new(arbitrary_call::<ArgsAsSlice>),
            Box::new(arbitrary_call::<ArgsLen>),
            Box::new(arbitrary_call::<ArgsIsEmpty>),
            Box::new(arbitrary_call::<ArgsGet>),
            Box::new(arbitrary_call::<ArgsGetN>),
            Box::new(arbitrary_call::<ArgsFirst>),
            Box::new(arbitrary_call::<ArgsLast>),
            Box::new(arbitrary_call::<ArgsIter>),
            Box::new(arbitrary_call::<ArgsCloneTo>),
                
            Box::new(arbitrary_call::<ArgsAsSliceMut>),
            Box::new(arbitrary_call::<ArgsGetMut>),
            Box::new(arbitrary_call::<ArgsGetNMut>),
            Box::new(arbitrary_call::<ArgsFirstMut>),
            Box::new(arbitrary_call::<ArgsLastMut>),
            Box::new(arbitrary_call::<ArgsIterMut>),
            Box::new(arbitrary_call::<ArgsRotateLeft>),
            Box::new(arbitrary_call::<ArgsRotateRight>),
            Box::new(arbitrary_call::<ArgsReverse>),
                
            Box::new(arbitrary_call::<ArgsAllocNew>),
            Box::new(arbitrary_call::<ArgsClone>),
            Box::new(arbitrary_call::<ArgsGrowWith>),
            Box::new(arbitrary_call::<ArgsGrow>),
            Box::new(arbitrary_call::<ArgsShrink>),
            Box::new(arbitrary_call::<ArgsPushFront>),
            Box::new(arbitrary_call::<ArgsPushNFront>),
            Box::new(arbitrary_call::<ArgsPushBack>),
            Box::new(arbitrary_call::<ArgsPushNBack>),
            Box::new(arbitrary_call::<ArgsPopFront>),
            Box::new(arbitrary_call::<ArgsPopNFront>),
            Box::new(arbitrary_call::<ArgsPopBack>),
            Box::new(arbitrary_call::<ArgsPopNBack>),
        ];

        // Construct the calls in an arbitrary order
        let mut calls = Vec::new();
        while !constructors.is_empty() {
            // Select an nth constructor and call it
            let nth = u.int_in_range(0 ..= constructors.len() - 1)?;
            let call = constructors.remove(nth)(u)?;
            calls.push(call);
        }

        Ok(Self { calls })
    }
}


fuzz_target!(|array_fn_calls: ArrayFnCalls| {
    // Note: This function can never fail because `Vec::new` can never fail
    let mut array = Array::alloc_new();
    
    // Execute the calls
    for mut array_fn_call in array_fn_calls.calls {
        array_fn_call.call(&mut array);
    }
});