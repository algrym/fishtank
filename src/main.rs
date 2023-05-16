use std::io;
use std::thread;
use std::time::Duration;

use console::{style, Term};

fn render() -> io::Result<()> {
    let term = Term::stdout();
    let (height, width) = term.size();

    term.set_title("Fishtank");
    term.hide_cursor()?;
    term.clear_screen()?;

    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut c: u8 = 0;

    let mut delta_x: i16 = 1;
    let mut delta_y: i16 = 1;

    loop {
        // Update locations
        if x >= width - 1 { delta_x = -1 }
        if y >= height - 1 { delta_y = -1 }

        if x == 0 { delta_x = 1 }
        if y == 0 { delta_y = 1 }

        if c == u8::MAX { c = 0 } else { c += 1 }

        x = x.wrapping_add_signed(delta_x);
        y = y.wrapping_add_signed(delta_y);

        // Render element
        term.move_cursor_to(usize::from(x), usize::from(y))?;
        term.write_str(&format!("{}", style("@").color256(c)))?;
        thread::sleep(Duration::from_millis(20));
        term.clear_chars(1)?;
    }
}

fn main() {
    render().unwrap();
}