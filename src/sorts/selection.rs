use crate::{Compare, Value};
use generator::{done, Gn};

/// # Safety
///
/// Trust me.
pub unsafe fn selection(list: *mut [Value]) -> impl Iterator<Item = Compare> {
    Gn::new_scoped_local(move |mut scope| {
        yield_!(scope, None);

        let list = unsafe { &mut *list };

        for i in 0..list.len() - 1 {
            let mut min_index = i;

            for j in i..list.len() {
                yield_!(scope, [i, min_index]);
                if list[j] < list[min_index] {
                    min_index = j;
                }
            }

            list.swap(i, min_index);
        }

        yield_!(scope, None);

        done!();
    })
}
