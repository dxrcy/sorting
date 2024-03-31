use crate::SortState;
use generator::Gn;

pub fn selection(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        for i in 0..list.len() - 1 {
            let mut min_index = i;

            for j in i..list.len() {
                if list[j] < list[min_index] {
                    min_index = j;
                }

                scope.yield_(SortState {
                    list: list.clone(),
                    just_compared: Some([i, min_index]),
                });
            }

            list.swap(i, min_index);
        }

        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        generator::done()
    })
}

