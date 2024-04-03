use std::ops::*;

use crate::Value;

pub struct Slice<'a> {
    whole: &'a [u32],
    start: usize,
    end: usize,
}
pub struct SliceMut<'a> {
    whole: &'a mut [u32],
    start: usize,
    end: usize,
}

impl<'a> Slice<'a> {
    pub fn new(whole: &'a [u32], start: usize, end: usize) -> Self {
        Self { whole, start, end }
    }

    pub fn with_bounds(&self, start: usize, end: usize) -> Self {
        Self {
            whole: self.whole,
            start: self.start + start,
            end: self.start + end,
        }
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.whole[self.start..self.end]
    }

    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> SliceMut<'a> {
    pub fn new(whole: &'a mut [u32], start: usize, end: usize) -> Self {
        Self { whole, start, end }
    }

    pub fn get_whole(&self) -> &[u32] {
        self.whole
    }
    pub fn get_whole_mut(&mut self) -> &mut [u32] {
        self.whole
    }

    pub fn as_slice(&self) -> &[Value] {
        &self.whole[self.start..self.end]
    }
    pub fn as_mut_slice(&mut self) -> &mut [Value] {
        &mut self.whole[self.start..self.end]
    }

    pub fn start(&self) -> usize {
        self.start
    }
    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a> Index<usize> for Slice<'a> {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}
impl<'a> Index<usize> for SliceMut<'a> {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}
impl<'a> IndexMut<usize> for SliceMut<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

macro_rules! impl_range {
    ( $( $ty:ty ),* $(,)? ) => {
        $(
            impl<'a> Index<$ty> for Slice<'a> {
                type Output = [Value];
                fn index(&self, index: $ty) -> &Self::Output {
                    &self.as_slice()[index]
                }
            }
            impl<'a> Index<$ty> for SliceMut<'a> {
                type Output = [Value];
                fn index(&self, index: $ty) -> &Self::Output {
                    &self.as_slice()[index]
                }
            }
            impl<'a> IndexMut<$ty> for SliceMut<'a> {
                fn index_mut(&mut self, index: $ty) -> &mut Self::Output {
                    &mut self.as_mut_slice()[index]
                }
            }
        )*
    };
}

impl_range![
    RangeFull,               // [..]
    Range<usize>,            // [x..y]
    RangeInclusive<usize>,   // [x..=y]
    RangeTo<usize>,          // [..x]
    RangeToInclusive<usize>, // [..=x]
    RangeFrom<usize>,        // [x..]
];
