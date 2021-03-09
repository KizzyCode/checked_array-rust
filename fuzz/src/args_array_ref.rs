use crate::ArrayFnCall;
use checked_array::{ ArrayRef, Array };
use arbitrary::{ Arbitrary, Result, Unstructured };


#[derive(Debug, Arbitrary)]
pub struct ArgsAsSlice;
impl ArrayFnCall for ArgsAsSlice {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.as_slice();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsLen;
impl ArrayFnCall for ArgsLen {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.len();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsIsEmpty;
impl ArrayFnCall for ArgsIsEmpty {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.is_empty();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsGet {
    index: usize
}
impl ArrayFnCall for ArgsGet {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.get(self.index);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsGetN {
    start: usize,
    end: usize
}
impl ArrayFnCall for ArgsGetN {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.get_n(self.start..self.end);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsFirst;
impl ArrayFnCall for ArgsFirst {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.first();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsLast;
impl ArrayFnCall for ArgsLast {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.last();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsIter;
impl ArrayFnCall for ArgsIter {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.iter();
    }
}


#[derive(Debug)]
pub struct ArgsCloneTo {
    target: Array<Vec<u8>>
}
impl<'a> Arbitrary<'a> for ArgsCloneTo {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let bytes = Vec::arbitrary(u)?;
        Ok(Self { target: Array::new(bytes) })
    }
}
impl ArrayFnCall for ArgsCloneTo {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        let _ = array.clone_to(&mut self.target);
    }
}