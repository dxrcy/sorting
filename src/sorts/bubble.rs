use crate::SortState;
use generator::Gn;

pub fn bubble(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        for i in 0..list.len() {
            for j in 0..list.len() - i - 1 {
                if list[j] > list[j + 1] {
                    list.swap(j, j + 1);
                }

                scope.yield_(SortState {
                    list: list.clone(),
                    just_compared: Some([j, j + 1]),
                });
            }
        }

        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        generator::done()
    })
}

