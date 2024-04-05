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
use std::{cmp::Ordering, io, process, thread, time::Duration};

use args::{Algorithm, Args};
use sorting::{hsl_to_rgb, is_sorted, sorts, Compare, ListRef, Value};

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Get size of list and rendered area
    let size = args.size.unwrap_or_else(|| {
        let (mut width, mut height) = terminal::size().unwrap();
        width /= 2; // 2 columns per value
        if !args.full_height {
            height -= 3; // Account for prompt height after completed
        }
        if height * 2 < width {
            width = height * 2;
        }
        width as usize
    });
    let height = (size + 1) / 2;

    // Create list
    let mut list: Vec<_> = (1..=size as Value).rev().collect();
    // Randomize list, unless `--reversed`
    if !args.reversed {
        let mut rng = rand::thread_rng();
        list.shuffle(&mut rng);
    }

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

    // Enable raw mode
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    execute!(stdout, Clear(ClearType::All), crossterm::cursor::Hide)?;

    'sort: for compare in iter {
        // Exit with keypress
        if event::poll(Duration::from_millis(1)).unwrap() {
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
                    // Ignore other terminal events
                    _ => (),
                }
            }
        }

        // Frame delay
        if args.delay > 0 {
            thread::sleep(Duration::from_millis(args.delay));
        }

        for (x, value) in list.iter().enumerate() {
            let value = *value;

            // Locally sorted, if:
            //   - Left value is exactly 1 more
            //   - OR right value is exactly 1 less
            //   - OR strictly increasing across adjacent values ( LEFT < THIS < RIGHT )
            // End values are not considered for last condition
            let is_locally_sorted = (x > 0 && list[x - 1] == value - 1)
                || (x < list.len() - 1 && list[x + 1] == value + 1)
                || ((x == 0 || list[x - 1] < value)
                    && (x == list.len() - 1 || list[x + 1] > value));

            // Two subcolumns (one character wide) per value column
            for subcolumn in 0..2 {
                // Use smoother gradient if locally sorted
                let subcolumn_hue = if is_locally_sorted && subcolumn == 1 {
                    0.5
                } else {
                    0.0
                };

                // Current value is being compared in this iteration
                let is_compared = compare.is_some_and(|[a, b]| a == x || b == x);

                // Set foreground color from list value
                let hue = (value as f64 + subcolumn_hue) * 360.0 / size as f64;
                let saturation = 100.0;
                let lightness = if is_compared { 100.0 } else { 50.0 }; // White if being compared
                let rgb = hsl_to_rgb(hue, saturation, lightness);
                queue!(stdout, SetForegroundColor(Color::from(rgb)),)?;

                // Move to bottom of printed area
                let full_height_offset = if args.full_height { 3 } else { 0 };
                queue!(
                    stdout,
                    SetForegroundColor(Color::from(rgb)),
                    cursor::MoveToRow(height as u16 + full_height_offset),
                )?;

                for y in 0..height {
                    // Re-align to column (print moves cursor)
                    queue!(stdout, cursor::MoveToColumn(x as u16 * 2 + subcolumn),)?;
                    if y > 0 || !args.full_height {
                        queue!(stdout, cursor::MoveUp(1))?;
                    }

                    // Compare current y position to value
                    let mut ordering = (y as isize).cmp(&((value as isize - 1) / 2));

                    // Don't divide subcolmn with different blocks unless locally sorted
                    if ordering == Ordering::Equal && !is_locally_sorted {
                        ordering = Ordering::Less;
                    }

                    ///   Empty whitespace
                    pub const QUARTERS_0: char = ' ';
                    /// ▂ One quarter block
                    pub const QUARTERS_1: char = '\u{2582}';
                    /// ▄ Half block
                    pub const QUARTERS_2: char = '\u{2584}';
                    /// ▆ Three quarter block
                    pub const QUARTERS_3: char = '\u{2586}';
                    /// █ Full block
                    pub const QUARTERS_4: char = '\u{2588}';

                    // Get modular index of subcolumn, to draw appropriate character
                    let column_type = (value + 1) % 2 * 2; // 0 or 2
                    let subcolumn_type = column_type + subcolumn as u32; // 0..=3

                    // Choose character to print
                    let chars = match ordering {
                        Ordering::Greater => QUARTERS_0,
                        Ordering::Less => QUARTERS_4,
                        Ordering::Equal => match subcolumn_type {
                            0 => QUARTERS_1,
                            1 => QUARTERS_2,
                            2 => QUARTERS_3,
                            _ => QUARTERS_4,
                        },
                    };
                    print!("{}", chars);
                }
            }
        }
    }

    // Disable raw mode
    execute!(
        stdout,
        cursor::MoveTo(0, height as u16 - if args.full_height { 1 } else { 0 }),
        cursor::Show,
        ResetColor,
    )?;
    terminal::disable_raw_mode()?;

    // Check list is sorted
    if !is_sorted(&list) {
        println!("\x1b[1;31mThe list is not sorted.\x1b[0m");
        process::exit(1);
    }

    Ok(())
}
