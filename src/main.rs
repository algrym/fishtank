use std::thread;
use std::time::Duration;

use console::{Emoji, Term};
use rand::Rng;
use rand::rngs::ThreadRng;

struct Mobile {
    x: u16,
    y: u16,
    delta_x: i16,
    delta_y: i16,
}

impl Mobile {
    fn update(&mut self, height: u16, width: u16) {
        if self.x >= width - 1 { self.delta_x = -1 }
        if self.y >= height - 1 { self.delta_y = -1 }

        if self.x == 0 { self.delta_x = 1 }
        if self.y == 0 { self.delta_y = 1 }

        self.x = self.x.wrapping_add_signed(self.delta_x);
        self.y = self.y.wrapping_add_signed(self.delta_y);
    }

    fn render(&mut self, term: &Term) {
        term.move_cursor_to(usize::from(self.x), usize::from(self.y)).ok();
        term.write_str(&format!("{}", Emoji("🦀", "@"))).ok();
    }

    pub fn new(mut rng: ThreadRng, height: u16, width: u16) -> Self {
        Mobile {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            delta_x: if rng.gen_bool(0.5) { 1 } else { -1 },
            delta_y: if rng.gen_bool(0.5) { 1 } else { -1 },
        }
    }
}

fn mob_runner() {
    let term = Term::stdout();
    let (height, width) = term.size();
    let rng = rand::thread_rng();

    term.set_title("Fishtank");
    term.hide_cursor().ok();
    term.clear_screen().ok();

    let mut mob_vec = Vec::new();
    for _i in 1..10 {
        mob_vec.push(Mobile::new(rng.clone(), height, width));
    }

    loop {
        for mob in mob_vec.iter_mut() {
            mob.update(height, width);
            mob.render(&term);
        }
        thread::sleep(Duration::from_millis(50));
        term.clear_screen().ok();
    }
}

fn main() {
    mob_runner();
}