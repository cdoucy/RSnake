pub struct SdlTexture<'a> {
    pub texture: sdl2::render::Texture<'a>,
}

impl<'a> SdlTexture<'a> {
    pub fn new(texture: sdl2::render::Texture<'a>) -> SdlTexture<'a> {
        SdlTexture {
            texture: texture
        }
    }
}