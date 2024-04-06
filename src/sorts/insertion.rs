algorithm!(insertion: |list, scope| {
    for i in 1..list.len() {
        let mut j = i;

        while j > 0 {
            yield_!(scope, [j - 1, j]);

            if list[j - 1] < list[j] {
                break;
            }

            list.swap(j, j - 1);
            j -= 1;
        }
    }
});
