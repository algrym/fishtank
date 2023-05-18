use std::thread;
use std::thread::Thread;
use std::time::Duration;

use console::{Emoji, Term};
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

const FISH_EMOJI: [&'static str; 15] = ["🦀", "🐟", "🐠", "🐡", "🐙", "🐬", "🦑", "🪼", "🦈", "🦞", "🦐", "🐌", "🐳", "🐋", "🦈"];

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

struct Mobile {
    x: u16,
    y: u16,
    delta_x: i16,
    delta_y: i16,
    icon: String,
    speed: u8,
    last_wait: u8,
}

impl Mobile {
    fn update(&mut self, mut rng: ThreadRng, height: u16, width: u16) {
        let mut rng = rand::thread_rng();
        if self.last_wait < self.speed {
            self.last_wait += 1;
            return;
        } else {
            self.last_wait = 0;
        }

        if self.x > width - 1 { self.delta_x = -1 }
        if self.y > height - 1 { self.delta_y = -1 }

        if self.x == 0 { self.delta_x = 1 }
        if self.y == 0 { self.delta_y = 1 }

        if rng.gen_bool(0.7) { self.x = self.x.wrapping_add_signed(self.delta_x); }
        if rng.gen_bool(0.3) { self.y = self.y.wrapping_add_signed(self.delta_y); }
    }

    fn render(&mut self, term: &Term) {
        term.move_cursor_to(usize::from(self.x), usize::from(self.y)).ok();
        term.write_str(&self.icon.to_string()).ok();
    }

    pub fn new(height: u16, width: u16) -> Self {
        let mut rng = rand::thread_rng();
        Mobile {
            icon: Emoji(FISH_EMOJI.choose(&mut rng).unwrap(), ".").to_string(),
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            delta_x: if rng.gen_bool(0.5) { 1 } else { -1 },
            delta_y: if rng.gen_bool(0.5) { 1 } else { -1 },
            speed: rng.gen_range(1..20),
            last_wait: 0,
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
        let mut rng = rand::thread_rng();
        for mob in mob_vec.iter_mut() {
            mob.update(rng.clone(), height, width);
            mob.render(&term);
        }
        thread::sleep(Duration::from_millis(10));
        term.clear_screen().ok();
    }
}

fn main() {
    mob_runner();
}