use generator::{done, Scope};

algorithm!(quick: |list, scope| {
    if list.is_empty() {
        done!();
    }

    yield_!(scope, None);

    let len = list.len();
    quick_sort_part(&mut scope, list, 0, (len - 1) as isize);

    yield_!(scope, None);
});

fn quick_sort_part(scope: &mut Scope<(), Compare>, list: &mut [Value], low: isize, high: isize) {
    if low < high {
        let p = partition(scope, list, low, high);
        quick_sort_part(scope, list, low, p - 1);
        quick_sort_part(scope, list, p + 1, high);
    }
}

fn partition(scope: &mut Scope<(), Compare>, list: &mut [Value], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        loop {
            yield_!(scope, [store_index as usize, pivot]);

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

            yield_!(scope, [last_index as usize, pivot]);

            if list[last_index as usize] <= list[pivot] {
                break;
            }
            last_index -= 1;
        }

        if last_index >= 0 {
            yield_!(scope, [store_index as usize, last_index as usize]);
        }

        if store_index >= last_index {
            break;
        }

        list.swap(store_index as usize, last_index as usize);
    }

    list.swap(store_index as usize, pivot);

    store_index
}
