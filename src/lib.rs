pub mod colors;
pub mod sorts;

#[cfg(test)]
mod tests;

pub type Value = u32;

pub type Compare = Option<[usize; 2]>;

pub fn is_sorted(list: &[Value]) -> bool {
    if list.is_empty() {}
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

pub fn hsl_to_rgb(mut h: f64, mut s: f64, mut l: f64) -> (u8, u8, u8) {
    h /= 360.0;
    s /= 100.0;
    l /= 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    (r, g, b)
}

struct Slice<'a> {
    whole: &'a [u32],
    start: usize,
    end: usize,
}

impl<'a> Slice<'a> {
    pub fn new(whole: &'a [u32], start: usize, end: usize) -> Slice<'a> {
        Slice { whole, start, end }
    }
    pub fn as_slice(&self) -> &[Value] {
        &self.whole[self.start..self.end]
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    pub fn start(&self) -> usize {
        self.start
    }
    // pub fn end(&self) -> usize {
    //     self.end
    // }
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
        )*
    };
}

use std::ops::*;
impl_range![
    RangeFull,               // [..]
    Range<usize>,            // [x..y]
    RangeInclusive<usize>,   // [x..=y]
    RangeTo<usize>,          // [..x]
    RangeToInclusive<usize>, // [..=x]
    RangeFrom<usize>,        // [x..]
];

impl<'a> Index<usize> for Slice<'a> {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}
