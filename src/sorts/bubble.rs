use crate::SortState;
use generator::{done, Gn};

pub fn bubble(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        yield_!(scope, list, None);

        for i in 0..list.len() {
            for j in 0..list.len() - i - 1 {
                if list[j] > list[j + 1] {
                    list.swap(j, j + 1);
                }

                yield_!(scope, list, [j, j+1]);
            }
        }

        yield_!(scope, list, None);
        done!();
    })
}

