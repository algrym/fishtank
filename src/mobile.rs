use console::{Emoji, Term};
use rand::Rng;
use rand::prelude::SliceRandom;

const FISH_EMOJI: [&'static str; 15] = ["🦀", "🐟", "🐠", "🐡", "🐙", "🐬", "🦑", "🪼", "🦈", "🦞", "🦐", "🐌", "🐳", "🐋", "🦈"];

pub struct Mobile {
    x: u16,
    y: u16,
    delta_x: i16,
    delta_y: i16,
    icon: String,
    speed: u8,
    last_wait: u8,
}

impl Mobile {
    pub fn update(&mut self, height: u16, width: u16) {
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

        if rng.gen_bool(0.8) { self.x = self.x.wrapping_add_signed(self.delta_x); }
        if rng.gen_bool(0.2) { self.y = self.y.wrapping_add_signed(self.delta_y); }
    }

    pub fn render(&mut self, term: &Term) {
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
