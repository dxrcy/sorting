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

    pub fn get_whole(&self) -> &[u32] {
        self.whole
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

impl<'a> From<&'a [Value]> for Slice<'a> {
    fn from(whole: &'a [Value]) -> Self {
        let end = whole.len();
        Self {
            whole,
            start: 0,
            end,
        }
    }
}
impl<'a> From<&'a mut [Value]> for SliceMut<'a> {
    fn from(whole: &'a mut [Value]) -> Self {
        let end = whole.len();
        Self {
            whole,
            start: 0,
            end,
        }
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
