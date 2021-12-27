extern crate olc_pixel_game_engine;
use crate::olc_pixel_game_engine as olc;
use std::collections::VecDeque;

struct FlappyBird {
    f_section_width: f64,
    list_section: VecDeque<u32>,
    f_level_position: f64,

    b_reset_game: bool,
    f_bird_position: f64,
    f_bird_velocity: f64,
    f_bird_acceleration: f64,
    f_gravity: f64,
}

impl FlappyBird {
    pub fn new() -> Self {
        FlappyBird {
            list_section: VecDeque::new(),
            b_reset_game: false,
            f_section_width: 0.0f64,
            f_bird_position: 0.0f64,
            f_bird_velocity: 0.0f64,
            f_bird_acceleration: 0.0f64,
            f_gravity: 100.0f64,
            f_level_position: 0.064,
        }
    }
}

impl olc::Application for FlappyBird {
    fn on_user_create(&mut self) -> Result<(), olc::Error> {
        self.list_section.clear();
        self.list_section.push_back(0u32);
        self.list_section.push_back(0);
        self.list_section.push_back(0);
        self.list_section.push_back(0);
        self.list_section.push_back(0);

        self.b_reset_game = true;
        self.f_section_width = olc::screen_width() as f64 / (self.list_section.len() - 1) as f64;

        Ok(())
    }

    fn on_user_update(&mut self, f_elapsed_time: f32) -> Result<(), olc::Error> {
        let spaceButtonState = olc::get_key(olc::Key::SPACE);
        if spaceButtonState.pressed && self.f_bird_velocity >= self.f_gravity / 10.0f64 {
            self.f_bird_acceleration = 0.6f64;
            self.f_bird_velocity = -self.f_gravity / 4.0f64;
        } else {
            self.f_bird_acceleration += self.f_gravity * (f_elapsed_time as f64);
        }

        if self.f_bird_acceleration >= self.f_gravity {
            self.f_bird_acceleration >= self.f_gravity;
        }

        self.f_bird_velocity += self.f_bird_acceleration * (f_elapsed_time as f64);
        self.f_bird_position += self.f_bird_velocity * (f_elapsed_time as f64);

        let available_height = olc::screen_height() as i32 - 100i32;
        self.f_level_position += 14.0f64 * (f_elapsed_time as f64);
        if self.f_level_position > self.f_section_width {
            self.f_level_position -= self.f_section_width;
            self.list_section.pop_front();

            let mut i: u32 = rand::random::<u32>() % (available_height as u32);
            if i <= 10 {
                i = 0;
            }
            self.list_section.push_back(i);
        }


        println!("section_width: {}, full_width={}", self.f_section_width, olc::screen_width());
        olc::clear(olc::BLACK);
        let width = (0.3f64 * self.f_section_width) as i32;
        let mut n_section = 0.0f64;
        for s in self.list_section.iter() {
            let s = *s;
            println!("s={}", s);
            if s != 0 {
                let x = (n_section * self.f_section_width - self.f_level_position as f64) as i32;
                olc::fill_rect(x, olc::screen_height() - (s as i32), width, s as i32, olc::GREEN);
                olc::fill_rect(x, 0i32, width, available_height - s as i32, olc::GREEN);
            }

            n_section = n_section + 1.0;
        }



        let n_bird_x = ((olc::screen_width() as f64 )/ 3.0f64) as i32;

        if self.f_bird_velocity > 0.0f64 {
            olc::draw_string(n_bird_x, self.f_bird_position as i32 + 0, "\\\\\\", olc::WHITE);
            olc::draw_string(n_bird_x, self.f_bird_position as i32 + 8, "<\\\\\\=Q", olc::WHITE);
        } else {
            olc::draw_string(n_bird_x, self.f_bird_position as i32 + 0, "<///=>Q", olc::WHITE);
            olc::draw_string(n_bird_x, self.f_bird_position as i32 + 8, "///", olc::WHITE);
        }

        Ok(())
    }

    fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
        Ok(())
    }
}

fn main() {
    let mut game = FlappyBird::new();

    olc::start("Flabby Bird", &mut game, 500, 250, 2, 2);
}
