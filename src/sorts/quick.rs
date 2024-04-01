use crate::{Value, SortState};
use generator::{done, Gn, Scope};

pub fn quick(mut list: Vec<Value>) -> impl Iterator<Item = SortState> {
    Gn::new_scoped(move |mut scope| {
        if list.is_empty() {
            done!();
        }

        unsafe {
            scope.yield_unsafe(SortState {
                list: list.to_vec(),
                just_compared: None,
            })
        };

        let len = list.len();
        quick_sort_part(&mut scope, &mut list, 0, (len - 1) as isize);

        unsafe {
            scope.yield_unsafe(SortState {
                list: list.to_vec(),
                just_compared: None,
            })
        };

        done!();
    })
}

fn quick_sort_part(scope: &mut Scope<(), SortState>, list: &mut [Value], low: isize, high: isize) {
    if low < high {
        let p = partition(scope, list, low, high);
        quick_sort_part(scope, list, low, p - 1);
        quick_sort_part(scope, list, p + 1, high);
    }
}

fn partition(scope: &mut Scope<(), SortState>, list: &mut [Value], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        loop {
            unsafe {
                scope.yield_unsafe(SortState {
                    list: list.to_vec(),
                    just_compared: Some([store_index as usize, pivot]),
                })
            };
            if list[store_index as usize] >= list[pivot] {
                break;
            }
            store_index += 1;
        }

        last_index -= 1;
        loop {
            if last_index < 0 {
                break;
            }
            unsafe {
                scope.yield_unsafe(SortState {
                    list: list.to_vec(),
                    just_compared: Some([last_index as usize, pivot]),
                })
            };
            if list[last_index as usize] <= list[pivot] {
                break;
            }
            last_index -= 1;
        }

        if last_index >= 0 {
            unsafe {
                scope.yield_unsafe(SortState {
                    list: list.to_vec(),
                    just_compared: Some([store_index as usize, last_index as usize]),
                })
            };
        }

        if store_index >= last_index {
            break;
        }

        list.swap(store_index as usize, last_index as usize);
    }

    list.swap(store_index as usize, pivot);

    store_index
}
