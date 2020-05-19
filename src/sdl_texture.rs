use crate::sdl_sprite::SdlSprite;

pub struct SdlTexture<'a> {
    pub texture: sdl2::render::Texture<'a>,
    pub sprites: Vec<SdlSprite>
}

impl<'a> SdlTexture<'a> {
    pub fn new(texture: sdl2::render::Texture<'a>) -> SdlTexture<'a> {
        SdlTexture {
            texture: texture,
            sprites: Vec::new()
        }
    }

    pub fn add_sprite(&mut self, sprite: SdlSprite) {
        self.sprites.push(sprite);
    }

    pub fn push_front(&mut self, sprite: SdlSprite) {
        self.sprites.insert(0, sprite);
    }

    pub fn pop_spr(&mut self) {
        self.sprites.pop();
    }
}