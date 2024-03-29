use rand::seq::SliceRandom;
use std::io::{self, Write};

use sorting::{colors::*, sorts::SelectionSort, Sorter, Value};

fn main() {
    const SIZE: usize = 40;
    const FRAME_TIME: u64 = 20;

    let mut list: Vec<_> = (1..=SIZE as Value).collect();
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);

    let iter = SelectionSort::new(&mut list);

    for state in iter {
        clear();

        for y in 0..SIZE {
            for (x, value) in state.list.iter().enumerate() {
                let h = x as f64 * 360.0 / SIZE as f64;
                let s = 100.0;
                let mut l = 50.0;

                if state.compare.is_some_and(|[a, b]| a == x || b == x) {
                    l = 100.0;
                    // l = (1.0 - ((y as f64 / SIZE as f64) * (*value as f64 / 10.0)).min(1.0)) * 50.0;
                }

                let (r, g, b) = hsl_to_rgb(h, s, l);
                print!("\x1b[38;2;{};{};{}m", r, g, b);

                if *value as usize >= SIZE - y {
                    print!("{}", "\u{2588}".repeat(2));
                } else {
                    print!("  ");
                }

                print!("\x1b[0m");
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(FRAME_TIME));
    }

    if !is_sorted(&list) {
        println!("{RED}{}{RESET}", "The list is not sorted");
    }
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn is_sorted(list: &[Value]) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    true
}

pub fn hsl_to_rgb(mut h: f64, mut s: f64, mut l: f64) -> (u8, u8, u8) {
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
