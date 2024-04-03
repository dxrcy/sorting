use crate::slice::{Slice, SliceMut};
use generator::Scope;

algorithm!(merge: |list, scope| {
    yield_!(scope, None);
    merge_sort(&mut scope, SliceMut::new(list, 0, list.len()));
    yield_!(scope, None);
});

fn merge_sort(scope: &mut Scope<(), Compare>, mut list: SliceMut) {
    let len = list.len();
    let midpoint = len / 2;

    if len <= 1 {
        return;
    }

    let start = list.start();

    merge_sort(
        scope,
        SliceMut::new(list.get_whole_mut(), start, start + midpoint),
    );
    merge_sort(
        scope,
        SliceMut::new(list.get_whole_mut(), start + midpoint, start + len),
    );

    let mut aux = list.as_slice().to_vec();

    let left = Slice::new(list.get_whole(), start, start + midpoint);
    let right = Slice::new(list.get_whole(), start + midpoint, start + len);

    merge_part(scope, left, right, aux.as_mut_slice());

    list.as_mut_slice().copy_from_slice(&aux);
}

fn merge_part(scope: &mut Scope<(), Compare>, left: Slice, right: Slice, aux: &mut [Value]) {
    assert_eq!(left.len() + right.len(), aux.len());

    let mut left_index = 0;
    let mut right_index = 0;
    let mut aux_index = 0;

    while left_index < left.len() && right_index < right.len() {
        yield_!(
            scope,
            [left.start() + left_index, right.start() + right_index]
        );

        if left[left_index] < right[right_index] {
            aux[aux_index] = left[left_index];
            aux_index += 1;
            left_index += 1;
        } else {
            aux[aux_index] = right[right_index];
            aux_index += 1;
            right_index += 1;
        }
    }

    if left_index < left.len() {
        aux[aux_index..].copy_from_slice(&left[left_index..]);
    }
    if right_index < right.len() {
        aux[aux_index..].copy_from_slice(&right[right_index..]);
    }
}
