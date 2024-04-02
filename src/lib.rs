pub mod colors;
pub mod sorts;

// use colors::*;

pub type Value = u32;

pub type Compare = Option<[usize; 2]>;

// #[derive(Debug)]
// pub struct SortState {
//     pub list: Vec<Value>,
//     pub just_compared: Compare,
// }

// pub trait Sorter<'a> {
//     fn next(&mut self) -> Option<SortState>;
// }

// impl std::fmt::Display for SortState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{DIM}{}{RESET} ", "_".repeat((self.list.len() + 1) * 4))?;
//         writeln!(f)?;
//         for (i, item) in self.list.iter().enumerate() {
//             write!(f, "   ")?;
//             if let Some([a, b]) = self.just_compared {
//                 if i == a {
//                     write!(f, "{BRIGHT}{UNDERLINE}{BLUE}")?;
//                 } else if i == b {
//                     write!(f, "{BRIGHT}{BLUE}")?;
//                 }
//             } else {
//                 write!(f, "{RED}")?;
//             }
//             write!(f, "{}{RESET}", item)?;
//         }
//         writeln!(f)?;
//
//         Ok(())
//     }
// }
