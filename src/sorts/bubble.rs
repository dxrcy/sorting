use crate::{Compare, SortState, Sorter, Value};

#[derive(Debug)]
pub struct Bubble<'a> {
    list: &'a mut [Value],
    i: usize,
    j: usize,
    last_swap: usize,
    prev_last_swap: usize,
    just_compared: Compare,
}

impl<'a> Iterator for Bubble<'a> {
    type Item = SortState;
    fn next(&mut self) -> Option<Self::Item> {
        Sorter::next(self)
    }
}

impl<'a> Bubble<'a> {
    pub fn new(list: &'a mut [Value]) -> Self {
        let last_swap = list.len() - 1;
        Self {
            list,
            i: 0,
            j: 0,
            last_swap,
            prev_last_swap: last_swap,
            just_compared: None,
        }
    }
}

impl<'a> Sorter<'a> for Bubble<'a> {
    fn next(&mut self) -> Option<SortState> {
        if self.just_compared.is_none() {
            if self.i >= self.list.len() {
                return None;
            }

            self.just_compared = Some([0, 0]);
            return Some(SortState {
                list: self.list.to_vec(),
                just_compared: None,
                did_swap: false,
                is_done: false,
            });
        }

        if self.j >= self.prev_last_swap {
            if self.prev_last_swap == 1 {
                self.just_compared = None;
                self.i = self.list.len();
                return Some(SortState {
                    list: self.list.to_vec(),
                    just_compared: None,
                    did_swap: false,
                    is_done: true,
                });
            }

            self.prev_last_swap = self.last_swap;

            self.i += 1;
            self.j = 0;
        }

        if self.i >= self.list.len() {
            self.just_compared = None;
            return Some(SortState {
                list: self.list.to_vec(),
                just_compared: None,
                did_swap: false,
                is_done: true,
            });
        }

        let mut did_swap = false;
        self.just_compared = Some([self.j, self.j + 1]);
        if self.list[self.j] > self.list[self.j + 1] {
            self.list.swap(self.j, self.j + 1);
            did_swap = true;
            self.last_swap = self.j;
        }

        self.j += 1;

        Some(SortState {
            list: self.list.to_vec(),
            just_compared: self.just_compared,
            did_swap,
            is_done: false,
        })
    }
}
