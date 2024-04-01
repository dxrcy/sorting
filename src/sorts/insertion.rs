use crate::SortState;
use generator::{done, Gn};

pub fn insertion(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        yield_!(scope, list, None);

        for i in 1..list.len() {
            let mut j = i;
            while j > 0 {
                yield_!(scope, list, [j-1, j]);

                if list[j - 1] < list[j] {
                    break;
                }

                list.swap(j, j - 1);
                j -= 1;
            }
        }

        yield_!(scope, list, None);
        done!();
    })
}

