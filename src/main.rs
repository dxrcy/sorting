use std::io::{self, BufRead, Write};

use sorting::{Compare, SortState, Sorter};

fn main() {
    let mut list = vec![7, 3, 6, 2, 9, 10, 1, 8, 5, 4];

    let iter = SelectionSort::new(&mut list);

    for state in iter {
        clear();

        for y in 0..10 {
            for (x, value) in state.list.iter().enumerate() {
                let h = x as f64 * 360.0 / 10.0;
                let s = 100.0;
                let mut l = 50.0;

                if state.compare.is_some_and(|[a, b]| a == x || b == x) {
                    l = (1.0 + ((y as f64 / 10.0) * (*value as f64 / 10.0)).min(1.0)) * 50.0;
                }

                let (r, g, b) = hsl_to_rgb(h, s, l);
                print!("\x1b[38;2;{};{};{}m", r, g, b);

                if *value >= 10 - y {
                    print!("{}", "\u{2588}".repeat(2));
                } else {
                    print!("  ");
                }

                print!("\x1b[0m");
            }
            println!();
        }

        // wait();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
fn wait() {
    io::stdin().lock().read_line(&mut String::new()).unwrap();
}

fn hsl_to_rgb(mut h: f64, mut s: f64, mut l: f64) -> (u8, u8, u8) {
    h /= 360.0;
    s /= 100.0;
    l /= 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    (r, g, b)
}

fn is_sorted(list: &[i32]) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

#[derive(Debug)]
struct SelectionSort<'a> {
    list: &'a mut [i32],
    i: usize,
    j: usize,
    min_index: usize,
    compare: Compare,
}

impl<'a> Sorter<'a> for SelectionSort<'a> {
    fn new(list: &'a mut [i32]) -> Self {
        Self {
            list,
            i: 0,
            j: 1,
            min_index: 0,
            compare: None,
        }
    }

    fn next(&mut self) -> Option<SortState> {
        if self.i >= self.list.len() - 1 {
            if self.compare.is_none() {
                return None;
            }

            self.compare = None;

            return Some(SortState {
                list: self.list.to_vec(),
                compare: None,
                did_swap: false,
                is_done: true,
            });
        }

        if self.compare.is_none() {
            self.compare = Some([0, 0]);

            return Some(SortState {
                list: self.list.to_vec(),
                compare: None,
                did_swap: false,
                is_done: false,
            });
        }

        self.compare = Some([self.j, self.min_index]);
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
            compare: self.compare,
            did_swap,
            is_done: false,
        })
    }
}

impl<'a> Iterator for SelectionSort<'a> {
    type Item = SortState;
    fn next(&mut self) -> Option<Self::Item> {
        Sorter::next(self)
    }
}
