use gfx;
use gfx::Device;

use image::{DynamicImage, GenericImage};

use errors::Res;


pub trait Texture {
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn offset_x(&self) -> u16;
    fn offset_y(&self) -> u16;
    fn handle(&self) -> gfx::TextureHandle;
    fn info(&self) -> &gfx::tex::TextureInfo;
    fn basic_texture(&self) -> &BasicTexture;

    fn get_measurements(&self) -> (f32, f32, f32, f32) {
        let info = self.info();
        (
            self.width() as f32 / info.width as f32, 
            self.height() as f32 / info.height as f32, 
            self.offset_x() as f32 / info.width as f32,
            self.offset_y() as f32 / info.height as f32,
        )
    }

    fn slice(&self, x: u16, y: u16, width: u16, height: u16) -> TextureSlice {
        TextureSlice {
            parent: self.basic_texture(),
            width: width,
            height: height,
            offset_x: self.offset_x() + x,
            offset_y: self.offset_y() + y,
        }
    }
}

pub struct BasicTexture {
    handle: gfx::TextureHandle,
    info: gfx::tex::TextureInfo,
}

pub struct TextureSlice<'a> {
    parent: &'a BasicTexture,
    width: u16,
    height: u16,
    offset_x: u16,
    offset_y: u16
}

impl BasicTexture {

    pub fn new(info: gfx::tex::TextureInfo, handle: gfx::TextureHandle) -> BasicTexture {
        BasicTexture {
            handle: handle,
            info: info,
        }
    }

    pub fn from_image(device: &mut gfx::GlDevice, image: &DynamicImage) -> Res<BasicTexture> {
        let (width, height) = image.dimensions();
        let texture_info = gfx::tex::TextureInfo {
            width: width as u16,
            height: height as u16,
            depth: 1,
            levels: 1,
            kind: gfx::tex::TextureKind::Texture2D,
            format: gfx::tex::RGBA8,
        };
        let image_info = texture_info.to_image_info();

        let texture = try!(device.create_texture(texture_info));

        match image.as_rgba8() {
            Some(buf) => {
                try!(device.update_texture(&texture, &image_info, buf.rawbuf()));
            },
            None => {
                try!(device.update_texture(&texture, &image_info, image.to_rgba().rawbuf()));
            }
        }

        Ok(BasicTexture::new(texture_info, texture))
    }
}

impl Texture for BasicTexture {
    fn width(&self) -> u16 { self.info.width as u16 }
    fn height(&self) -> u16 { self.info.height as u16 }
    fn offset_x(&self) -> u16 { 0 }
    fn offset_y(&self) -> u16 { 0 }
    fn handle(&self) -> gfx::TextureHandle { self.handle }
    fn info(&self) -> &gfx::tex::TextureInfo { &self.info }
    fn basic_texture(&self) -> &BasicTexture { self }
}

impl<'a> Texture for TextureSlice<'a> {
    fn width(&self) -> u16 { self.width }
    fn height(&self) -> u16 { self.height }
    fn offset_x(&self) -> u16 { self.offset_x }
    fn offset_y(&self) -> u16 { self.offset_y }
    fn handle(&self) -> gfx::TextureHandle { self.parent.handle() }
    fn info(&self) -> &gfx::tex::TextureInfo { self.parent.info() }
    fn basic_texture(&self) -> &BasicTexture { self.parent }
}
