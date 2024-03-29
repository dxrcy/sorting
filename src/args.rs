use std::fmt::Display;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
pub struct Args {
    #[clap(default_value_t = Algorithm::Selection)]
    pub algorithm: Algorithm,

    #[clap(short, long, default_value = "20")]
    pub size: usize,

    #[clap(short, long, default_value = "0")]
    pub frame_duration: u64,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Algorithm {
    Selection,
    Insertion,
    Bubble,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_possible_value().unwrap().get_name())
    }
}
