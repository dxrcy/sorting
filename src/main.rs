fn main() {
    let mut list = vec![2, 1, 4, 3];
    // let mut list = vec![4, 3, 2, 1];

    let iter = SelectionSort::new(&mut list);

    for step in iter {
        println!("{}", step);
    }

    println!("{:?}", list);
    println!("sorted: {}", is_sorted(&list).to_string().to_uppercase());
}

fn is_sorted(list: &[i32]) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

type Compare = Option<[usize; 2]>;

#[derive(Debug)]
struct SortState {
    list: Vec<i32>,
    compare: Compare,
    did_swap: bool,
    is_done: bool,
}

trait Sorter<'a> {
    fn new(list: &'a mut [i32]) -> Self;

    fn next(&mut self) -> Option<SortState>;
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

impl<'a> std::fmt::Display for SortState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // writeln!(f, "{:?}", self.list)?;
        // writeln!(f, "    i: {}", self.i)?;
        // writeln!(f, "    j: {}", self.j)?;
        // writeln!(f, "    min_index: {:?}", self.min_index)?;
        // writeln!(f, "    compare: {:?}", self.compare)?;

        write!(f, "{DIM}{}{RESET} ", "_".repeat((self.list.len() + 1) * 4))?;
        writeln!(f)?;
        for (i, item) in self.list.iter().enumerate() {
            write!(f, "   ")?;
            if let Some([a, b]) = self.compare {
                let color = if self.did_swap { YELLOW } else { BLUE };
                if i == a {
                    write!(f, "{BRIGHT}{color}")?;
                } else if i == b {
                    write!(f, "{BRIGHT}{UNDERLINE}{color}")?;
                }
            } else {
                if self.is_done {
                write!(f, "{BRIGHT}{GREEN}")?;
                } else { write!(f, "{RED}")?; };
            }
            write!(f, "{}{RESET}", item)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";
const BRIGHT: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const ITALIC: &str = "\x1b[3m";
const UNDERLINE: &str = "\x1b[4m";
const BLINK: &str = "\x1b[5m";
const REVERSE: &str = "\x1b[7m";
const STRIKE: &str = "\x1b[9m";
