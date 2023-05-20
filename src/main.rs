use std::thread;
use std::time::Duration;

use console::Term;
use mobile::Mobile;

mod mobile;
mod direction;

fn mob_runner() {
    let term = Term::stdout();
    let (height, width) = term.size();

    term.set_title("Fishtank");
    term.hide_cursor().ok();
    term.clear_screen().ok();

    let mut mob_vec = Vec::new();
    for _i in 1..10 {
        mob_vec.push(Mobile::new(height, width));
    }

    loop {
        for mob in mob_vec.iter_mut() {
            mob.update(height, width);
            mob.render(&term);
        }
        thread::sleep(Duration::from_millis(10));
        term.clear_screen().ok();
    }
}

fn main() {
    mob_runner();
}
