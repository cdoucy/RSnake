use sdl2::rect::{Rect, Point};

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

use Direction::{UP, DOWN, LEFT, RIGHT};

#[derive(Copy, Clone)]
pub struct SdlSprite {
    pub texture_rect: Rect,
    pub sprite_rect: Rect,
    direction: Direction,
    is_static: bool
}

impl SdlSprite {
    pub fn new(texture_rect: Rect, coord: Point, scale: u32) -> SdlSprite {
        SdlSprite {
            texture_rect: texture_rect,
            sprite_rect: Rect::from_center(
                coord,
                texture_rect.width() * scale,
                texture_rect.height() * scale
            ),
            direction: UP,
            is_static: false
        }
    }

    pub fn move_sprite(&mut self, velocity: i32) {
        if !self.is_static {
            match self.direction {
                UP => self.sprite_rect.set_y(self.sprite_rect.y() - velocity),
                DOWN => self.sprite_rect.set_y(self.sprite_rect.y() + velocity),
                LEFT => self.sprite_rect.set_x(self.sprite_rect.x() - velocity),
                RIGHT => self.sprite_rect.set_x(self.sprite_rect.x() + velocity)
            }
        }
    }

    pub fn set_dir_x(&mut self, direction: Direction, x: i32) {
        self.direction = direction;
        self.texture_rect.set_x(x);
    }

    pub fn get_dir(&self) -> Direction {
        self.direction
    }

    pub fn get_pos(&self) -> (i32, i32) {
        (self.sprite_rect.x(), self.sprite_rect.y())
    }

    pub fn set_x_y_texture(&mut self, x: i32, y: i32) {
        self.texture_rect.set_x(x);
        self.texture_rect.set_y(y);
    }

    pub fn clone(&self) -> SdlSprite {
        SdlSprite {
            texture_rect: self.texture_rect,
            sprite_rect: self.sprite_rect,
            direction: self.direction,
            is_static: self.is_static
        }
    }

    pub fn set_pos(&mut self, x: i32, y: i32) {
        self.sprite_rect.set_x(x);
        self.sprite_rect.set_y(y);
    }

    pub fn overlap(&self, other: &SdlSprite) -> bool {
        let (x, y) = self.get_pos();
        let (x_o, y_o) = other.get_pos();

        let w = self.sprite_rect.width() as i32;
        let h = self.sprite_rect.height() as i32;

        let w_o = other.sprite_rect.width() as i32;
        let h_o = other.sprite_rect.height() as i32;

        if x > x_o + w_o || x_o > x + w {
            return false;
        }

        if y > y_o + h_o || y_o > y + h {
            return false;
        }

        return true;
    }
}