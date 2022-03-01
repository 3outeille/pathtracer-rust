use nalgebra::Vector3;

// pub struct TextureInfo;

pub trait TextureMaterial: {
    // fn get_texture(&self,  point: Vector3<f32>) -> TextureInfo;
    fn get_texture(&self,  point: Vector3<f32>) -> Vector3<f32>;
}

pub struct UniformTexture {
    pub color: Vector3<f32>
}

impl UniformTexture {
    pub fn new(color_arg: Vector3<f32>) -> Self {
        Self {
            color: color_arg
        }
    }
}

impl TextureMaterial for UniformTexture {
    fn get_texture(&self, point: Vector3<f32>) -> Vector3<f32> {
        return self.color;
    }
}