pub mod colors;
pub mod sorts;

use colors::*;

pub type Value = u32;

pub type Compare = Option<[usize; 2]>;

#[derive(Debug)]
pub struct SortState {
    pub list: Vec<Value>,
    pub compare: Compare,
    pub did_swap: bool,
    pub is_done: bool,
}

pub trait Sorter<'a> {
    fn new(list: &'a mut [Value]) -> Self;

    fn next(&mut self) -> Option<SortState>;
}

impl<'a> std::fmt::Display for SortState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{DIM}{}{RESET} ", "_".repeat((self.list.len() + 1) * 4))?;
        writeln!(f)?;
        for (i, item) in self.list.iter().enumerate() {
            write!(f, "   ")?;
            if let Some([a, b]) = self.compare {
                let color = if self.did_swap { YELLOW } else { BLUE };
                if i == a {
                    write!(f, "{BRIGHT}{UNDERLINE}{color}")?;
                } else if i == b {
                    write!(f, "{BRIGHT}{color}")?;
                }
            } else {
                if self.is_done {
                    write!(f, "{BRIGHT}{GREEN}")?;
                } else {
                    write!(f, "{RED}")?;
                };
            }
            write!(f, "{}{RESET}", item)?;
        }
        writeln!(f)?;

        Ok(())
    }
}