use crate::ArrayFnCall;
use checked_array::{ ArrayMut, Array };
use arbitrary::Arbitrary;


#[derive(Debug, Arbitrary)]
pub struct ArgsAsSliceMut;
impl ArrayFnCall for ArgsAsSliceMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.as_slice_mut();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsGetMut {
    index: usize
}
impl ArrayFnCall for ArgsGetMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.get_mut(self.index);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsGetNMut {
    start: usize,
    end: usize
}
impl ArrayFnCall for ArgsGetNMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.get_n_mut(self.start..self.end);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsFirstMut;
impl ArrayFnCall for ArgsFirstMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.first_mut();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsLastMut;
impl ArrayFnCall for ArgsLastMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.last_mut();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsIterMut;
impl ArrayFnCall for ArgsIterMut {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.iter_mut();
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsRotateLeft {
    steps: usize
}
impl ArrayFnCall for ArgsRotateLeft {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.rotate_left(self.steps);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsRotateRight {
    steps: usize
}
impl ArrayFnCall for ArgsRotateRight {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.rotate_right(self.steps);
    }
}


#[derive(Debug, Arbitrary)]
pub struct ArgsReverse;
impl ArrayFnCall for ArgsReverse {
    fn call(&mut self, array: &mut Array<Vec<u8>>) {
        array.reverse();
    }
}