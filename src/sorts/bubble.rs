use crate::{Compare, SortState, Sorter, Value};

#[derive(Debug)]
pub struct Bubble<'a> {
    list: &'a mut [Value],
    i: usize,
    j: usize,
    did_any_swaps: bool,
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
        Self {
            list,
            i: 0,
            j: 0,
            did_any_swaps: false,
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

        if self.j >= self.list.len() - 1 {
            if !self.did_any_swaps {
                self.just_compared = None;
                self.i = self.list.len();
                return Some(SortState {
                    list: self.list.to_vec(),
                    just_compared: None,
                    did_swap: false,
                    is_done: true,
                });
            }

            self.i += 1;
            self.j = 0;
            self.did_any_swaps = false;
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
            self.did_any_swaps = true;
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
