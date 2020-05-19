use sdl2::image::LoadTexture;
use crate::sdl_texture::SdlTexture;
use crate::sdl_sprite::SdlSprite;

pub struct Renderer
{
    pub canvas: sdl2::render::WindowCanvas
}

impl Renderer {
    pub fn new(context: &sdl2::Sdl, title: &str, width: u32, height: u32) -> Renderer {
        let video_subsystem = context.video().expect(&sdl2::get_error());
        let window = video_subsystem.window(title, width, height)
            .position_centered()
            .build()
            .expect(&sdl2::get_error());
        let canvas = window.into_canvas()
            .present_vsync()
            .build()
            .expect(&sdl2::get_error());
        Renderer {
            canvas: canvas
        }
    }

    pub fn render(&mut self, bg_color: sdl2::pixels::Color, textures: &SdlTexture, sprites: &Vec<SdlSprite>) {
        self.canvas.set_draw_color(bg_color);
        self.canvas.clear();
        for s in sprites.iter() {
            self.canvas.copy(&textures.texture, s.texture_rect, s.sprite_rect).expect(&sdl2::get_error());
        }
        self.canvas.present();
    }
}

pub struct TextureWrapper
{
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>
}

impl<'a> TextureWrapper {
    pub fn new(canvas: &sdl2::render::WindowCanvas) -> TextureWrapper {
        TextureWrapper {
            texture_creator: canvas.texture_creator()
        }
    }

    pub fn load_texture(&self, filepath: &str) -> SdlTexture {
        let texture = self.texture_creator.load_texture(filepath).expect(&sdl2::get_error());
        SdlTexture::new(texture)
    }
}