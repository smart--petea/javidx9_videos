extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct ExampleProgram {}

impl olc::Application for ExampleProgram {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }

    fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
        olc::clear(olc::BLACK);
        olc::draw_string(40, 40, "Hello, World!", olc::WHITE)?;
        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

fn main() {
    let mut example = ExampleProgram {};
    olc::start("Hello, World!", &mut example, 200, 100, 4, 4).unwrap();
}
