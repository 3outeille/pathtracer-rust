
pub struct TextureInfo;

pub trait TextureMaterial {
    fn get_texture(&self, x: u32, y: u32) -> TextureInfo;
}

pub struct UniformTexture {
}

impl UniformTexture {
    pub fn new() -> Self {
        Self {}
    }
}

impl TextureMaterial for UniformTexture {
    fn get_texture(&self, x: u32, y: u32) -> TextureInfo {
        todo!()
    }
}