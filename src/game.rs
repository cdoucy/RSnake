use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Instant};

use rand::Rng;

use crate::sdl_wrapper::Renderer;
use crate::sdl_wrapper::TextureWrapper;
use crate::sdl_sprite::SdlSprite;
use crate::sdl_sprite::Direction::{UP, DOWN, LEFT, RIGHT};

pub struct Game {
    renderer: Renderer,
    events: sdl2::EventPump,
    texture_w: TextureWrapper,
    snake: Vec<SdlSprite>,
    apple: SdlSprite
}

impl Game {
    pub fn new() -> Game {
        let context = sdl2::init().expect(&sdl2::get_error());
        sdl2::image::init(sdl2::image::InitFlag::PNG).expect(&sdl2::get_error());
        let renderer = Renderer::new(&context, "Rust Snake", 800, 600);
        let tw = TextureWrapper::new(&renderer.canvas);
        let events = context.event_pump().expect(&sdl2::get_error());

        Game {
            renderer: renderer,
            events: events,
            texture_w: tw,
            snake: Vec::new(),
            apple: SdlSprite::new(Rect::new(48, 0, 8, 8), Point::new(400, 250), 3)
        }
    }

    pub fn run(&mut self) {
        let texture = self.texture_w.load_texture("snake.png");
        Game::init_sprites(&mut self.snake);
        let mut clock = Instant::now();
        let mut rng = rand::thread_rng();

        'running: loop {
            for ev in self.events.poll_iter() {
                match Game::manage_event(&ev, &mut self.snake[0]) {
                    false => break 'running,
                    _ => {}
                }
            }
            if clock.elapsed().as_millis() > 30 {
                match Game::is_done(&self.snake) {
                    true => break 'running,
                    _ => {}
                }
                Game::manage_snake(&mut self.snake, &mut self.apple, &mut rng);
                self.snake.push(self.apple);
                self.renderer.render(Color::RGB(66, 50, 100), &texture, &self.snake);
                self.snake.pop();
                clock = Instant::now();
            }
        }
    }

    fn is_done(sprites: &Vec<SdlSprite>) -> bool {
        let (x, y) = sprites[0].get_pos();

        if x >= 800 || x <= 0 || y >= 600 || y <= 0 {
            return true;
        }

        for i in 1..sprites.len() {
            if sprites[0].overlap(&sprites[i]) {
                return true;
            }
        }
        false
    }

    fn manage_snake(sprites: &mut Vec<SdlSprite>, apple: &mut SdlSprite, rng: &mut rand::rngs::ThreadRng) {
        let mut head = sprites[0].clone();

        head.move_sprite(16);
        sprites.insert(0, head);
        sprites[1].set_x_y_texture(40, 24);

        if !sprites[0].overlap(apple) {
            sprites.pop();
        } else {
            let mut check = true;

            while check {

                let x = rng.gen_range(24, 776);
                let y = rng.gen_range(24, 576);
                apple.set_pos(x, y);

                for s in sprites.iter() {
                    check = s.overlap(apple);
                }
            }
        }
    }

    fn init_sprites(sprites: &mut Vec<SdlSprite>) {
        sprites.push(SdlSprite::new(Rect::new(8, 24, 8, 8), Point::new(400, 300), 2));
        sprites.push(SdlSprite::new(Rect::new(40, 24, 8, 8), Point::new(400, 324), 2));
        sprites.push(SdlSprite::new(Rect::new(40, 24, 8, 8), Point::new(400, 348), 2));
        sprites.push(SdlSprite::new(Rect::new(40, 24, 8, 8), Point::new(400, 372), 2));
    }

    fn manage_event(ev: &sdl2::event::Event, head: &mut SdlSprite) -> bool {
        let mut ret = true;
        match ev {
            Event::Quit {..} => ret = false,
            Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                match head.get_dir() {
                    DOWN => {},
                    _ => head.set_dir_x(UP, 8),
                }
            },
            Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                match head.get_dir() {
                    RIGHT => {},
                    _ => head.set_dir_x(LEFT, 16),
                }
            },
            Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                match head.get_dir() {
                    UP => {},
                    _ => head.set_dir_x(DOWN, 24)
                }
            },
            Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                match head.get_dir() {
                    LEFT => {},
                    _ => head.set_dir_x(RIGHT, 32)
                }
            },
            _ => {}
        }
        ret
    }
}