use crate::SortState;
use generator::Gn;

pub fn insertion(mut list: Vec<u32>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        for i in 1..list.len() {
            let mut j = i;
            while j > 0 {
                scope.yield_(SortState {
                    list: list.clone(),
                    just_compared: Some([j - 1, j]),
                });

                if list[j - 1] < list[j] {
                    break;
                }

                list.swap(j, j - 1);
                j -= 1;
            }
        }

        scope.yield_(SortState {
            list: list.clone(),
            just_compared: None,
        });

        generator::done()
    })
}

