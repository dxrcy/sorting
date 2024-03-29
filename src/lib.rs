use colors::*;

pub type Compare = Option<[usize; 2]>;

#[derive(Debug)]
pub struct SortState {
    pub list: Vec<i32>,
    pub compare: Compare,
    pub did_swap: bool,
    pub is_done: bool,
}

pub trait Sorter<'a> {
    fn new(list: &'a mut [i32]) -> Self;

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

#[allow(dead_code)]
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const BRIGHT: &str = "\x1b[1m";
    pub const DIM: &str = "\x1b[2m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    pub const BLINK: &str = "\x1b[5m";
    pub const REVERSE: &str = "\x1b[7m";
    pub const STRIKE: &str = "\x1b[9m";
}
