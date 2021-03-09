use crate::{ ALLOC_MAX, ArrayFnCall };
use checked_array::{ ArrayAlloc, Array };
use arbitrary::{ Arbitrary, Result, Unstructured };
use std::cmp;


#[derive(Debug, Arbitrary)]
pub struct ArgsAllocNew;
impl ArrayFnCall for ArgsAllocNew {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        if let Ok(new) = Array::alloc_new() {
            *array = new;
        }
    }
}


#[derive(Debug)]
pub struct ArgsClone {
    source: Array<Vec<u8>>
}
impl<'a> Arbitrary<'a> for ArgsClone {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bytes = Vec::arbitrary(u)?;
        Ok(Self { source: Array::new(bytes) })
    }
}
impl ArrayFnCall for ArgsClone {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        if let Ok(new) = Array::alloc_clone(&self.source) {
            *array = new;
        }
    }
}


#[derive(Debug)]
pub struct ArgsGrowWith {
    len: usize,
    init: u8
}
impl<'a> Arbitrary<'a> for ArgsGrowWith {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let len = cmp::min(usize::arbitrary(u)?, *ALLOC_MAX);
        let init = u8::arbitrary(u)?;
        Ok(Self { len, init })
    }
}
impl ArrayFnCall for ArgsGrowWith {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.grow_with(self.len, || self.init);
    }
}


#[derive(Debug)]
pub struct ArgsGrow {
    len: usize
}
impl<'a> Arbitrary<'a> for ArgsGrow {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let len = cmp::min(usize::arbitrary(u)?, *ALLOC_MAX);
        Ok(Self { len })
    }
}
impl ArrayFnCall for ArgsGrow {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.grow(self.len);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsShrink {
    len: usize
}
impl ArrayFnCall for ArgsShrink {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.shrink(self.len);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPushFront {
    element: u8
}
impl ArrayFnCall for ArgsPushFront {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.push_front(self.element);
    }
}


#[derive(Debug)]
pub struct ArgsPushNFront {
    source: Array<Vec<u8>>
}
impl<'a> Arbitrary<'a> for ArgsPushNFront {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bytes = Vec::arbitrary(u)?;
        Ok(Self { source: Array::new(bytes) })
    }
}
impl ArrayFnCall for ArgsPushNFront {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.push_n_front(&mut self.source);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPushBack {
    element: u8
}
impl ArrayFnCall for ArgsPushBack {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.push_back(self.element);
    }
}


#[derive(Debug)]
pub struct ArgsPushNBack {
    source: Array<Vec<u8>>
}
impl<'a> Arbitrary<'a> for ArgsPushNBack {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bytes = Vec::arbitrary(u)?;
        Ok(Self { source: Array::new(bytes) })
    }
}
impl ArrayFnCall for ArgsPushNBack {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.push_n_back(&mut self.source);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPopFront;
impl ArrayFnCall for ArgsPopFront {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.pop_front();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPopNFront {
    len: usize
}
impl ArrayFnCall for ArgsPopNFront {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.pop_n_front(self.len);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPopBack;
impl ArrayFnCall for ArgsPopBack {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.pop_back();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsPopNBack {
    len: usize
}
impl ArrayFnCall for ArgsPopNBack {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.pop_n_back(self.len);
    }
}