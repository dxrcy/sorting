use crate::Value;

algorithm!(pancake_selection: |list, scope| {
    for i in 0..list.len() {
        let min = min(&list[i..]);

        flip(&mut list[i + min..]);
        yield_!(scope, [i + min, list.len() - 1]);

        flip(&mut list[i..]);
        yield_!(scope, [i, list.len() - 1]);
    }
});

fn min(array: &[Value]) -> usize {
    let mut min = 0;
    for i in 1..array.len() {
        if array[i] < array[min] {
            min = i;
        }
    }
    min
}

fn flip(array: &mut [Value]) {
    for i in 0..array.len() / 2 {
        array.swap(i, array.len() - 1 - i);
    }
}
