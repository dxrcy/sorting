use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;

use std::env;
use std::io::{self, Write};

use sorting::{colors::*, sorts, Sorter, Value};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    let size: usize = match args.next().map(|arg| arg.parse()) {
        Some(Ok(x)) => x,
        _ => 20,
    };
    let frame_duration: u64 = match args.next().map(|arg| arg.parse()) {
        Some(Ok(x)) => x,
        _ => 0,
    };

    let mut list: Vec<_> = (1..=size as Value).collect();
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);

    let iter = sorts::InsertionSort::new(&mut list);

    terminal::enable_raw_mode()?;

    execute!(io::stdout(), Clear(ClearType::All), crossterm::cursor::Hide,)?;

    'sort: for state in iter {
        if event::poll(std::time::Duration::from_millis(1)).unwrap() {
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

        for (x, value) in state.list.iter().enumerate() {
            let h = *value as f64 * 360.0 / size as f64;
            let s = 100.0;
            let l = if state.just_compared.is_some_and(|[a, b]| a == x || b == x) {
                100.0
            } else {
                50.0
            };

            let (r, g, b) = hsl_to_rgb(h, s, l);

            execute!(io::stdout(), SetForegroundColor(Color::Rgb { r, g, b }))?;

            for y in 0..size {
                execute!(
                    io::stdout(),
                    cursor::MoveTo(x as u16 * 2, size as u16 - y as u16 - 1)
                )?;

                if y < *value as usize {
                    print!("{}", "\u{2588}".repeat(2));
                } else {
                    print!("  ");
                }
            }
        }

        if frame_duration > 0 {
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(frame_duration));
        }
    }

    execute!(
        io::stdout(),
        cursor::MoveTo(0, size as u16),
        cursor::Show,
        ResetColor,
    )?;
    terminal::disable_raw_mode()?;

    if !is_sorted(&list) {
        println!("{BRIGHT}{RED}{}{RESET}", "The list is not sorted\r");
    }

    Ok(())
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
