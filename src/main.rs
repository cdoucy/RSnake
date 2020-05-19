extern crate rand;

mod sdl_sprite;
mod sdl_texture;
mod sdl_wrapper;
mod game;

fn main() {
    let mut game = game::Game::new();
    game.run();
}