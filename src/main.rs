use std::thread;
use std::time::Duration;

use console::Term;
use mobile::Mobile;

mod mobile;

enum _Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl _Direction {
    fn _value(&self) -> (i8, i8) {
        match *self {
            _Direction::North => (0, 1),
            _Direction::NorthEast => (1, 1),
            _Direction::East => (1, 0),
            _Direction::SouthEast => (1, -1),
            _Direction::South => (0, -1),
            _Direction::SouthWest => (-1, -1),
            _Direction::West => (-1, 0),
            _Direction::NorthWest => (-1, 1),
        }
    }
}

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
