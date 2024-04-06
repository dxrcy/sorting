algorithm!(shell: |list, scope| {
    let len = list.len();
    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            let temp = list[i];
            let mut j = i;

            while j >= gap {
                yield_!(scope, [j, i]);
                if list[j - gap] <= temp {
                    break;
                }

                list[j] = list[j - gap];
                j -= gap;
            }

            list[j] = temp;
        }

        gap /= 2;
    }
});


