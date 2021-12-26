extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;
use std::collections::VecDeque;

struct FlappyBird {
    list_section: VecDeque<i32>,
    b_reset_game: bool,
    f_section_width: f64,
}

impl FlappyBird {
    pub fn new() -> Self {
        FlappyBird {
            list_section: VecDeque::new(),
            b_reset_game: false,
            f_section_width: 0.0,
        }
    }
}

impl olc::Application for FlappyBird {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.list_section.clear();
        self.list_section.push_back(0);
        self.list_section.push_back(0);
        self.list_section.push_back(0);
        self.list_section.push_back(0);

        self.b_reset_game = true;
        self.f_section_width = olc::screen_width() as f64 / (self.list_section.len() - 1) as f64;

        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

fn main() {
    let mut game = FlappyBird::new();

    olc::start("Flabby Bird", &mut game, 80, 48, 16, 16);
}
