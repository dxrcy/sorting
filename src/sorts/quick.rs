use crate::slice::SliceMut;
use generator::{done, Scope};

algorithm!(quick: |list, scope| {
    if list.is_empty() {
        done!();
    }
    quick_part(&mut scope, SliceMut::from(list));
});

fn quick_part(scope: &mut Scope<(), Compare>, mut list: SliceMut) {
    let len = list.len();
    if len <= 1 {
        return;
    }

    let pivot_index = partition(scope, &mut list);
    quick_part(scope, slice!(&mut list, 0, pivot_index));
    quick_part(scope, slice!(&mut list, pivot_index + 1, len));
}

fn partition(scope: &mut Scope<(), Compare>, list: &mut SliceMut) -> usize {
    let len = list.len();

    let pivot_index = len / 2;

    list.as_mut_slice().swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..list.len() - 1 {
        yield_!(scope, [list.start() + j, list.start() + len - 1]);

        if list[j] < list[len - 1] {
            list.as_mut_slice().swap(i, j);
            i += 1;
        }
    }

    list.as_mut_slice().swap(i, len - 1);

    i
}
