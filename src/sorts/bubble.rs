algorithm!(bubble: |list, scope| {
    yield_!(scope, None);

    for i in 0..list.len() {
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }

            yield_!(scope, [j, j + 1]);
        }
    }

    yield_!(scope, None);
});
