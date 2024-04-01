use crate::SortState;
use generator::{done, Gn};

pub fn selection(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        yield_!(scope, list, None);

        for i in 0..list.len() - 1 {
            let mut min_index = i;

            for j in i..list.len() {
                if list[j] < list[min_index] {
                    min_index = j;
                }
                yield_!(scope, list, [i, min_index]);
            }

            list.swap(i, min_index);
        }

        yield_!(scope, list, None);
        done!();
    })
}
