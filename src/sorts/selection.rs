use generator::done;

algorithm!(selection: |list, scope| {
    if list.len() <= 1 {
        done!();
    }

    for i in 0..list.len() - 1 {
        let mut min_index = i;

        for j in i..list.len() {
            yield_!(scope, [j, min_index]);

            if list[j] < list[min_index] {
                min_index = j;
            }
        }

        list.swap(i, min_index);
    }
});
