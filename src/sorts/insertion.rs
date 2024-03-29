use crate::{Compare, SortState, Sorter, Value};

#[derive(Debug)]
pub struct InsertionSort<'a> {
    list: &'a mut [Value],
    i: usize,
    j: usize,
    just_compared: Compare,
}

impl<'a> Iterator for InsertionSort<'a> {
    type Item = SortState;
    fn next(&mut self) -> Option<Self::Item> {
        Sorter::next(self)
    }
}

impl<'a> Sorter<'a> for InsertionSort<'a> {
    fn new(list: &'a mut [Value]) -> Self {
        Self {
            list,
            i: 1,
            j: 1,
            just_compared: None,
        }
    }

    fn next(&mut self) -> Option<SortState> {
        if self.just_compared.is_none() {
            if self.i >= self.list.len() + 1 {
                return None;
            }
            if self.i >= self.list.len() {
                self.i += 1;
                return Some(SortState {
                    list: self.list.to_vec(),
                    just_compared: None,
                    did_swap: false,
                    is_done: true,
                });
            }

            self.just_compared = Some([0, 0]);
            return Some(SortState {
                list: self.list.to_vec(),
                just_compared: None,
                did_swap: false,
                is_done: false,
            });
        }

        if self.j > 0 {
            self.just_compared = Some([self.j - 1, self.j]);
        }

        let mut did_swap = false;
        if self.j == 0 || self.list[self.j - 1] <= self.list[self.j] {
            self.i += 1;
            self.j = self.i;
        } else {
            self.list.swap(self.j, self.j - 1);
            self.j -= 1;
            did_swap = true;
        }

        if self.i >= self.list.len() {
            let just_compared = self.just_compared;
            self.just_compared = None;
            return Some(SortState {
                list: self.list.to_vec(),
                just_compared,
                did_swap,
                is_done: false,
            });
        }

        Some(SortState {
            list: self.list.to_vec(),
            just_compared: self.just_compared,
            did_swap,
            is_done: false,
        })
    }
}
