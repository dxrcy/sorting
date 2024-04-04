use std::fmt::Display;

use clap::{Parser, ValueEnum};

/// Animate sorting algorithms in the terminal
#[derive(Parser)]
pub struct Args {
    /// Sorting algorithm to demo
    #[clap(default_value_t = Algorithm::Quick)]
    pub algorithm: Algorithm,

    /// Length of list
    ///
    /// Proportional to width of printed area.
    /// Defaults to fit terminal size.
    #[clap(short, long)]
    pub size: Option<usize>,

    /// Duration to wait between frames
    #[clap(short, long, default_value = "0")]
    pub delay: u64,

    /// Render with full height of terminal
    ///
    /// By default, height will be reduced by 3 rows.
    #[clap(short, long)]
    pub full_height: bool,

    /// Start with reversed list, not random
    #[clap(short, long)]
    pub reversed: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Algorithm {
    Bubble,
    Insertion,
    Merge,
    Quick,
    Selection,
    Random,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}
