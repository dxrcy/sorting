algorithm!(bubble: |list, scope| {
    for i in 0..list.len() {
        let mut swaps = 0;
        for j in 0..list.len() - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
                swaps += 1;
            }

            yield_!(scope, [j, j + 1]);
        }
        if swaps == 0 {
            break;
        }
    }
});
