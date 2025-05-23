#[macro_use]
mod macros;

define_algorithms! {
    0 | Bubble => bubble,
    1 | Insertion => insertion,
    2 | Merge => merge,
    3 | Quick => quick,
    4 | Selection => selection,
    5 | Shell => shell,
    6 | Bogo => bogo,
    7 | PancakeSelection => pancake_selection,
}

/// `Slice` and `SliceMut` custom slice types
pub mod slice;

#[cfg(test)]
mod tests;

/// List item value
pub type Value = u32;
/// Current list comparison, yielded from generator functions
pub type Compare = Option<[usize; 2]>;

/// 'Smart' pointer for sharing a mutable reference
///
/// # Safety
///
/// Trust me.
pub struct ListRef(*mut [Value]);

impl ListRef {
    /// Create a 'smart' pointer from a mutable slice
    pub fn from(list: &mut [Value]) -> Self {
        Self(list as *mut [Value])
    }
    /// Dereference pointer as mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [Value] {
        unsafe { &mut *self.0 }
    }
}

/// Check if list is sorted
pub fn is_sorted(list: &[Value]) -> bool {
    if list.is_empty() {
        return true;
    }
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

/// Convert HSL to RGB
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
