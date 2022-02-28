use dyn_clone::{clone_trait_object, DynClone};

pub trait TextureMaterial: DynClone {
    fn get_texture(&self, x: u32, y: u32) -> TextureInfo;
}

clone_trait_object!(TextureMaterial);


#[derive(Clone)]
pub struct TextureInfo;

#[derive(Clone)]
pub struct UniformTexture;

// impl UniformTexture {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

impl TextureMaterial for UniformTexture {
    fn get_texture(&self, x: u32, y: u32) -> TextureInfo {
        todo!()
    }
}