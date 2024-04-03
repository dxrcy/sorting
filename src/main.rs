mod args;

use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::{io, process, thread, time};

use args::{Algorithm, Args};
use sorting::{colors::*, hsl_to_rgb, is_sorted, sorts, Compare, ListRef, Value};

fn main() -> io::Result<()> {
    let args = Args::parse();

    let size = args.size;
    let height = args.size;

    let mut list: Vec<_> = (1..=size as Value).collect();
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);

    let ptr = ListRef::from(&mut list);

    let iter: Box<dyn Iterator<Item = Compare>> = match args.algorithm {
        Algorithm::Bubble => Box::new(sorts::bubble(ptr)),
        Algorithm::Insertion => Box::new(sorts::insertion(ptr)),
        Algorithm::Merge => Box::new(sorts::merge(ptr)),
        Algorithm::Quick => Box::new(sorts::quick(ptr)),
        Algorithm::Selection => Box::new(sorts::selection(ptr)),
    };

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

            queue!(stdout, SetForegroundColor(Color::Rgb { r, g, b }))?;

            for y in 0..height {
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16 * 3, height as u16 - y as u16 - 1)
                )?;

                if y + 1 > *value as usize {
                    print!("   ");
                } else if y + 1 == *value as usize {
                    print!("🭈🭆🭂");
                } else {
                    print!("\u{2588}\u{2588}\u{2588}");
                }
            }
        }

        if args.frame_duration > 0 {
            thread::sleep(time::Duration::from_millis(args.frame_duration));
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
