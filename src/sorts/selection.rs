use crate::{Compare, SortState, Sorter, Value};

#[derive(Debug)]
pub struct SelectionSort<'a> {
    list: &'a mut [Value],
    i: usize,
    j: usize,
    min_index: usize,
    just_compared: Compare,
}

impl<'a> Iterator for SelectionSort<'a> {
    type Item = SortState;
    fn next(&mut self) -> Option<Self::Item> {
        Sorter::next(self)
    }
}

impl<'a> Sorter<'a> for SelectionSort<'a> {
    fn new(list: &'a mut [Value]) -> Self {
        Self {
            list,
            i: 0,
            j: 1,
            min_index: 0,
            just_compared: None,
        }
    }

    fn next(&mut self) -> Option<SortState> {
        if self.i >= self.list.len() - 1 {
            if self.just_compared.is_none() {
                return None;
            }

            self.just_compared = None;

            return Some(SortState {
                list: self.list.to_vec(),
                just_compared: None,
                did_swap: false,
                is_done: true,
            });
        }

        if self.just_compared.is_none() {
            self.just_compared = Some([0, 0]);

            return Some(SortState {
                list: self.list.to_vec(),
                just_compared: None,
                did_swap: false,
                is_done: false,
            });
        }

        self.just_compared = Some([self.j, self.min_index]);
        if self.list[self.j] < self.list[self.min_index] {
            self.min_index = self.j;
        }

        let mut did_swap = false;
        self.j += 1;
        if self.j >= self.list.len() {
            self.list.swap(self.i, self.min_index);
            did_swap = true;

            self.i += 1;
            self.j = self.i + 1;
            self.min_index = self.i;
        }

        Some(SortState {
            list: self.list.to_vec(),
            just_compared: self.just_compared,
            did_swap,
            is_done: false,
        })
    }
}
