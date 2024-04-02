use crate::{Compare, Value};
use generator::{done, Gn};

/// # Safety
///
/// Trust me.
pub unsafe fn selection(list: *mut [Value]) -> impl Iterator<Item = Compare> {
    Gn::new_scoped_local(move |mut scope| {
        unsafe { scope.yield_unsafe(None) };

        let list = unsafe { &mut *list };

        for i in 0..list.len() - 1 {
            let mut min_index = i;

            for j in i..list.len() {
                if list[j] < list[min_index] {
                    min_index = j;
                }

                unsafe { scope.yield_unsafe(Some([i, min_index])) };
            }

            list.swap(i, min_index);
        }

        unsafe { scope.yield_unsafe(None) };

        done!();
    })
}
