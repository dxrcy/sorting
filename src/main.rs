mod args;

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use rand::{seq::SliceRandom, Rng};
use std::{io, process, thread, time};

use args::{Algorithm, Args};
use sorting::{colors::*, hsl_to_rgb, is_sorted, sorts, Compare, ListRef, Value};

fn main() -> io::Result<()> {
    let args = Args::parse();

    let size = args.size.unwrap_or_else(|| {
        let (mut width, mut height) = terminal::size().unwrap();
        width /= 2; // 2 columns per value
        height -= 3; // Account for prompt height after completed
        if height * 2 < width {
            width = height * 2;
        }
        width as usize
    });
    let height = (size + 1) / 2;

    let mut list: Vec<_> = (1..=size as Value).collect();
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);

    /// Local macro, see below
    macro_rules! choose_algorithm {
        (
            $( $index:literal | $variant:ident => $fn:tt ),*
            $(,)?
        ) => {{
            // Get 'smart' pointer for list -- possibly unsaf
            let ptr = ListRef::from(&mut list);
            // Get generator from algorithm enum variant
            let iter: Box<dyn Iterator<Item = Compare>> = match args.algorithm {
                $(
                    Algorithm::$variant => Box::new(sorts::$fn(ptr)),
                )*
                Algorithm::Random => Box::new({
                    let mut rng = rand::thread_rng();
                    // Get index literal of last pattern
                    let count = *[ $( $index ),* ].last().unwrap();
                    // Get function from random index
                    match rng.gen_range(0..=count) {
                        $(
                            $index => sorts::$fn(ptr),
                        )*
                        _ => unreachable!("macro is broken"),
                    }
                })
            };
            iter
        }};
    }

    let iter = choose_algorithm!(
        0 | Bubble => bubble,
        1 | Insertion => insertion,
        2 | Merge => merge,
        3 | Quick => quick,
        4 | Selection => selection,
    );

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, Clear(ClearType::All), crossterm::cursor::Hide,)?;

    'sort: for compare in iter {
        if event::poll(time::Duration::from_millis(1)).unwrap() {
            if let Ok(event) = event::read() {
                match event {
                    Event::Key(KeyEvent {
                        code, modifiers, ..
                    }) => {
                        if code == KeyCode::Esc
                            || code == KeyCode::Char('q')
                            || (code == KeyCode::Char('c')
                                && modifiers.contains(KeyModifiers::CONTROL))
                        {
                            break 'sort;
                        }
                    }
                    _ => break,
                }
            }
        }

        for (x, value) in list.iter().enumerate() {
            let h = *value as f64 * 360.0 / size as f64;
            let s = 100.0;
            let l = if compare.is_some_and(|[a, b]| a == x || b == x) {
                100.0
            } else {
                50.0
            };

            let (r, g, b) = hsl_to_rgb(h, s, l);

            queue!(
                stdout,
                SetForegroundColor(Color::Rgb { r, g, b }),
                // Move to bottom of printed area
                cursor::MoveToRow(height as u16),
            )?;

            for y in 0..height {
                queue!(
                    stdout,
                    // Re-align to column (print moves cursor)
                    cursor::MoveToColumn(x as u16 * 2),
                    cursor::MoveUp(1),
                )?;

                // Signed distance to top of block
                let distance = *value as isize / 2 - y as isize;
                let is_odd = value % 2 == 1;

                if distance > 0 {
                    print!("\u{2588}\u{2588}"); // Full block
                } else if distance == 0 && is_odd {
                    print!("\u{2584}\u{2584}"); // Half block
                } else {
                    print!("  "); // Blank
                }
            }
        }

        if args.delay > 0 {
            thread::sleep(time::Duration::from_millis(args.delay));
        }
    }

    execute!(
        stdout,
        cursor::MoveTo(0, height as u16),
        cursor::Show,
        ResetColor,
    )?;
    terminal::disable_raw_mode()?;

    if !is_sorted(&list) {
        println!("{BRIGHT}{RED}The list is not sorted.{RESET}");
        process::exit(1);
    }

    Ok(())
}
