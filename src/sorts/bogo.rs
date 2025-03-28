use crate::{is_sorted, Value};

algorithm!(bogo: |list, scope| {
    while !is_sorted(list) {
        shuffle_list(list);
        yield_!(scope, [list.len(), list.len()]);
    }
});

fn shuffle_list(list: &mut [Value]) {
    for i in (1..list.len()).rev() {
        list.swap(i, random_int() % (i + 1));
    }
}

fn random_int() -> usize {
    std::time::Instant::now().elapsed().as_nanos() as usize
}
